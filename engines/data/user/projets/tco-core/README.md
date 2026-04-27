# TCO Core - Acquisition de Données Multi-Capteurs

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![C++](https://img.shields.io/badge/C++-Arduino-orange.svg)
![Python](https://img.shields.io/badge/python-3.11+-green.svg)
![Raspberry Pi](https://img.shields.io/badge/Raspberry%20Pi-ARM-red.svg)
![InfluxDB](https://img.shields.io/badge/InfluxDB-1.8-purple.svg)
![Grafana](https://img.shields.io/badge/Grafana-Latest-orange.svg)
![Prometheus](https://img.shields.io/badge/Prometheus-Latest-red.svg)
![Loki](https://img.shields.io/badge/Loki-2.9-blue.svg)
![Promtail](https://img.shields.io/badge/Promtail-2.9-blue.svg)
![Docker](https://img.shields.io/badge/Docker-Compose-blue.svg)

Système d'acquisition temps réel : Arduino → Gateway Python → InfluxDB → Grafana

## Structure

```
.
├── daq/
│   ├── daq.ino                      # Firmware principal
│   ├── data_packet/
│   │   └── DataPacketV2.h          # Structure paquet binaire
│   ├── sensor_drivers/
│   │   ├── MPU6050Sensor.h         # Driver MPU6050
│   │   ├── DS3231Sensor.h          # Driver DS3231 RTC
│   │   └── NEO6MGPSSensor.h        # Driver GPS NEO-6M
│   └── utilities/
│       └── crc8_checksum.h         # Utilitaires CRC8
├── gateway/
│   ├── main.py                      # Gateway Python
│   └── gateway.service              # Service systemd
├── analysis/
│   └── influx_spectral.py
├── flake.nix                        # Environnement Nix (dev)
├── requirements.txt                 # Dépendances Python (production)
└── infra/
    ├── docker-compose.yml
    ├── loki.yml
    └── prometheus.yml
```

## Installation

### Arduino

```bash
arduino-cli lib install "Adafruit MPU6050" "Adafruit Unified Sensor" "RTClib" "TinyGPSPlus"
arduino-cli compile --fqbn arduino:avr:uno daq/
arduino-cli upload -p /dev/ttyUSB0 --fqbn arduino:avr:uno daq/
```

### Gateway

**Installation**

```bash
# Option 1 : venv (production - recommandé)
cd gateway
python3 -m venv venv
source venv/bin/activate
pip install -r ../requirements.txt

cd ..
sudo cp gateway/gateway.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now gateway

# Option 2 : Nix (développement)
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
nix develop
```

### Infra (InfluxDB + Grafana + monitoring)

```bash
cd infra
docker compose up -d
```

## Câblage

| Arduino | MPU6050 | DS3231 | GPS Neo-6M |
|---------|---------|--------|------------|
| 5V | VCC | VCC | VCC |
| GND | GND | GND | GND |
| A4 (SDA) | SDA | SDA | - |
| A5 (SCL) | SCL | SCL | - |
| Pin 3 | - | - | TX (GPS) |
| Pin 4 | - | - | RX (GPS) |

## Format Paquet

40 bytes binaire : SYNC (2) + ms (4) + ts (4) + accels (6) + gyros (6) + temp (2) + lat (4) + lon (4) + sats (1) + CRC (1) + padding (6)

## InfluxDB

- Database: `tpu_db`
- Measurement: `tpu_sensors`
- Fréquence: 100 Hz
- Champs: ax_g, ay_g, az_g, gx_dps, gy_dps, gz_dps, temp_c, lat, lon, sats, uptime_ms
