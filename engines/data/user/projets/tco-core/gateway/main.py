import serial
import struct
import time
import signal
import sys
import logging
import os
from influxdb import InfluxDBClient

NODE_ID = "tpu-core"
SENSOR_TYPE = "mpu6050_neo6m"

SERIAL_PORT = '/dev/ttyUSB0'
BAUD_RATE = 115200
TIMEOUT_SEC = 1.0

DB_HOST = 'localhost'
DB_PORT = 8086
DB_NAME = 'tpu_db'

BATCH_SIZE = 500
FLUSH_INTERVAL_SEC = 5.0

PACKET_FMT = '<BBIIhhhhhhhiiBB6x' 
PACKET_SIZE = struct.calcsize(PACKET_FMT)

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s | %(levelname)s | %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger(__name__)

running = True

def signal_handler(sig, frame):
    global running
    logger.info("Arrêt demandé")
    running = False

signal.signal(signal.SIGINT, signal_handler)
signal.signal(signal.SIGTERM, signal_handler)

def connect_db():
    try:
        client = InfluxDBClient(host=DB_HOST, port=DB_PORT, database=DB_NAME)
        client.ping()
        logger.info(f"DB connectée: {DB_HOST}:{DB_PORT}")
        return client
    except Exception as e:
        logger.error(f"Echec DB: {e}")
        sys.exit(1)

def connect_serial():
    try:
        ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=TIMEOUT_SEC)
        ser.reset_input_buffer()
        logger.info(f"Série ouvert: {SERIAL_PORT} @ {BAUD_RATE}")
        return ser
    except Exception as e:
        logger.error(f"Echec série: {e}")
        sys.exit(1)

def main():
    logger.info(f"Gateway démarrée (Packet: {PACKET_SIZE} bytes)")
    
    db_client = connect_db()
    ser = connect_serial()
    
    buffer = b''
    batch_buffer = []
    total_packets = 0
    sync_errors = 0
    last_stat_time = time.time()
    last_flush_time = time.time()

    gps_valid_count = 0
    gps_invalid_count = 0
    last_gps_log_time = time.time()

    while running:
        try:
            chunk = ser.read(ser.in_waiting or 256)
            if not chunk:
                continue
            buffer += chunk
        except Exception as e:
            logger.error(f"Erreur lecture série: {e}")
            break

        while len(buffer) >= PACKET_SIZE:
            if buffer[0] == 0xAA and buffer[1] == 0x55:
                packet_raw = buffer[:PACKET_SIZE]
                buffer = buffer[PACKET_SIZE:] 
                
                try:
                    data = struct.unpack(PACKET_FMT, packet_raw)
                    
                    sats = int(data[13])
                    lat_fix = int(data[11])
                    lon_fix = int(data[12])
                    
                    point = {
                        "measurement": "tpu_sensors",
                        "tags": {"node": NODE_ID, "sensor": SENSOR_TYPE},
                        "time": int(time.time() * 1e9), 
                        "fields": {
                            "ax_g": data[4] / 1000.0,
                            "ay_g": data[5] / 1000.0,
                            "az_g": data[6] / 1000.0,
                            "gx_dps": data[7] / 1000.0,
                            "gy_dps": data[8] / 1000.0,
                            "gz_dps": data[9] / 1000.0,
                            "temp_c": data[10] / 100.0,
                            "sats": sats,
                            "uptime_ms": int(data[2])
                        }
                    }
                    
                    if lat_fix != 0 and lon_fix != 0:
                        point["fields"]["lat"] = lat_fix / 1000000.0
                        point["fields"]["lon"] = lon_fix / 1000000.0
                        gps_valid_count += 1
                    else:
                        gps_invalid_count += 1

                    batch_buffer.append(point)
                    total_packets += 1

                    if len(batch_buffer) >= BATCH_SIZE:
                        db_client.write_points(batch_buffer, time_precision='n')
                        batch_buffer = [] 

                except Exception as e:
                    logger.warning(f"Erreur décodage: {e}")
                    sync_errors += 1
            else:
                buffer = buffer[1:]
                sync_errors += 1

        current_time = time.time()
        if current_time - last_flush_time >= FLUSH_INTERVAL_SEC and batch_buffer:
            try:
                db_client.write_points(batch_buffer, time_precision='n')
                batch_buffer = []
            except Exception as e:
                logger.warning(f"Echec write_points: {e}")
            last_flush_time = current_time

        if current_time - last_stat_time > 5.0:
            duration = current_time - last_stat_time
            rate = (total_packets / duration) if duration > 0 else 0
            
            if current_time - last_gps_log_time >= 30.0:
                total_gps = gps_valid_count + gps_invalid_count
                if total_gps > 0:
                    gps_valid_pct = (gps_valid_count / total_gps * 100)
                    if gps_valid_pct < 10.0:
                        logger.warning(f"GPS: {gps_valid_count}/{total_gps} fixes valides ({gps_valid_pct:.1f}%)")
                    else:
                        logger.info(f"GPS: {gps_valid_count}/{total_gps} fixes valides ({gps_valid_pct:.1f}%)")
                last_gps_log_time = current_time
                gps_valid_count = 0
                gps_invalid_count = 0
            
            logger.info(f"Taux: {rate:6.2f} Hz | Total: {total_packets} | Sync Errors: {sync_errors}")
            total_packets = 0
            last_stat_time = current_time

    if batch_buffer:
        try:
            db_client.write_points(batch_buffer, time_precision='n')
        except Exception:
            pass

    if ser.is_open:
        ser.close()
    logger.info("Gateway arrêtée")

if __name__ == '__main__':
    main()
