#include <Wire.h>
#include "data_packet/DataPacketV2.h"
#include "sensor_drivers/MPU6050Sensor.h"
#include "sensor_drivers/DS3231Sensor.h"
#include "sensor_drivers/NEO6MGPSSensor.h"
#include "utilities/crc8_checksum.h"

#define SERIAL_BAUD_RATE 115200
const uint16_t SAMPLING_FREQUENCY_HZ = 100;
const uint32_t SAMPLING_INTERVAL_MICROSECONDS = 1000000 / SAMPLING_FREQUENCY_HZ;

static const int GPS_RX_PIN = 3;
static const int GPS_TX_PIN = 4;
static const uint32_t GPS_BAUD_RATE = 9600;

MPU6050Sensor mpu6050_sensor;
DS3231Sensor ds3231_rtc_sensor;
NEO6MGPSSensor neo6m_gps_sensor(GPS_RX_PIN, GPS_TX_PIN, GPS_BAUD_RATE);

bool ds3231_rtc_initialized = false;
uint32_t cached_unix_timestamp_seconds = 0;
uint8_t rtc_read_counter = 0;
int32_t cached_latitude_microdegrees = 0;
int32_t cached_longitude_microdegrees = 0;
uint8_t cached_satellite_count = 0;
uint32_t last_gps_cache_update_milliseconds = 0;

void setup() {
  Serial.begin(SERIAL_BAUD_RATE);
  Wire.begin();
  Wire.setClock(400000);
  delay(1000);

  ds3231_rtc_initialized = ds3231_rtc_sensor.begin();
  
  if (!mpu6050_sensor.begin()) {
    while (1) delay(100);
  }

  neo6m_gps_sensor.begin();
}

void loop() {
  static uint32_t last_sampling_timestamp_microseconds = 0;
  uint32_t current_timestamp_microseconds = micros();
  uint32_t current_timestamp_milliseconds = millis();
  
  neo6m_gps_sensor.process(20);

  if ((uint32_t)(current_timestamp_milliseconds - last_gps_cache_update_milliseconds) >= 200) {
    last_gps_cache_update_milliseconds = current_timestamp_milliseconds;
    int32_t new_latitude = neo6m_gps_sensor.getLatitude();
    int32_t new_longitude = neo6m_gps_sensor.getLongitude();
    uint8_t new_satellite_count = neo6m_gps_sensor.getSatellites();
    
    if (new_latitude != 0 && new_longitude != 0) {
      cached_latitude_microdegrees = new_latitude;
      cached_longitude_microdegrees = new_longitude;
    }
    if (new_satellite_count > 0) {
      cached_satellite_count = new_satellite_count;
    }
  }

  uint32_t elapsed_microseconds_since_last_sample;
  if (current_timestamp_microseconds >= last_sampling_timestamp_microseconds) {
    elapsed_microseconds_since_last_sample = current_timestamp_microseconds - last_sampling_timestamp_microseconds;
  } else {
    elapsed_microseconds_since_last_sample = (0xFFFFFFFF - last_sampling_timestamp_microseconds) + current_timestamp_microseconds;
  }
  
  if (elapsed_microseconds_since_last_sample >= SAMPLING_INTERVAL_MICROSECONDS) {
    last_sampling_timestamp_microseconds += SAMPLING_INTERVAL_MICROSECONDS;
    
    if (rtc_read_counter == 0 && ds3231_rtc_initialized) {
      cached_unix_timestamp_seconds = ds3231_rtc_sensor.getUnixTime();
    }
    rtc_read_counter++;
    if (rtc_read_counter >= 10) rtc_read_counter = 0;

    neo6m_gps_sensor.processExtra(10);

    int16_t acceleration_x_millig, acceleration_y_millig, acceleration_z_millig;
    int16_t gyroscope_x_millidegrees_per_second, gyroscope_y_millidegrees_per_second, gyroscope_z_millidegrees_per_second;
    int16_t temperature_centidegrees_celsius;
    mpu6050_sensor.read(acceleration_x_millig, acceleration_y_millig, acceleration_z_millig, 
                        gyroscope_x_millidegrees_per_second, gyroscope_y_millidegrees_per_second, gyroscope_z_millidegrees_per_second, 
                        temperature_centidegrees_celsius);

    DataPacketV2 data_packet{};
    data_packet.s0 = SYNC0; 
    data_packet.s1 = SYNC1;
    data_packet.ms = current_timestamp_milliseconds;
    data_packet.ts = cached_unix_timestamp_seconds;
    data_packet.ax = acceleration_x_millig;
    data_packet.ay = acceleration_y_millig;
    data_packet.az = acceleration_z_millig;
    data_packet.gx = gyroscope_x_millidegrees_per_second;
    data_packet.gy = gyroscope_y_millidegrees_per_second;
    data_packet.gz = gyroscope_z_millidegrees_per_second;
    data_packet.temp = temperature_centidegrees_celsius;
    data_packet.lat_fix = cached_latitude_microdegrees;
    data_packet.lon_fix = cached_longitude_microdegrees;
    data_packet.sats = cached_satellite_count;
    data_packet.crc = computeCRC8XOR((uint8_t*)&data_packet, 33);
    
    Serial.write((uint8_t*)&data_packet, sizeof(data_packet));
  }
}
