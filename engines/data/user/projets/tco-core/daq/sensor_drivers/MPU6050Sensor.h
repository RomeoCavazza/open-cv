#pragma once
#include <Wire.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>

class MPU6050Sensor {
public:
  bool begin() {
    const uint8_t I2C_ADDRESS_PRIMARY = 0x68;
    const uint8_t I2C_ADDRESS_SECONDARY = 0x69;
    
    if (!mpu6050_device.begin(I2C_ADDRESS_PRIMARY)) {
      if (!mpu6050_device.begin(I2C_ADDRESS_SECONDARY)) {
        return false;
      }
    }
    mpu6050_device.setAccelerometerRange(MPU6050_RANGE_2_G);
    mpu6050_device.setGyroRange(MPU6050_RANGE_250_DEG);
    mpu6050_device.setFilterBandwidth(MPU6050_BAND_44_HZ);
    return true;
  }

  void read(int16_t& acceleration_x_millig, 
            int16_t& acceleration_y_millig, 
            int16_t& acceleration_z_millig,
            int16_t& gyroscope_x_millidegrees_per_second,
            int16_t& gyroscope_y_millidegrees_per_second,
            int16_t& gyroscope_z_millidegrees_per_second,
            int16_t& temperature_centidegrees_celsius) {
    sensors_event_t acceleration_event, gyroscope_event, temperature_event;
    mpu6050_device.getEvent(&acceleration_event, &gyroscope_event, &temperature_event);
    
    const float MILLIG_PER_G = 101.93f;
    const float MILLIDEGREES_PER_SECOND_PER_DEGREE_PER_SECOND = 57296.0f / 1000.0f;
    const float CENTIDEGREES_PER_DEGREE = 100.0f;
    
    acceleration_x_millig = (int16_t)(acceleration_event.acceleration.x * MILLIG_PER_G);
    acceleration_y_millig = (int16_t)(acceleration_event.acceleration.y * MILLIG_PER_G);
    acceleration_z_millig = (int16_t)(acceleration_event.acceleration.z * MILLIG_PER_G);
    gyroscope_x_millidegrees_per_second = (int16_t)(gyroscope_event.gyro.x * MILLIDEGREES_PER_SECOND_PER_DEGREE_PER_SECOND);
    gyroscope_y_millidegrees_per_second = (int16_t)(gyroscope_event.gyro.y * MILLIDEGREES_PER_SECOND_PER_DEGREE_PER_SECOND);
    gyroscope_z_millidegrees_per_second = (int16_t)(gyroscope_event.gyro.z * MILLIDEGREES_PER_SECOND_PER_DEGREE_PER_SECOND);
    temperature_centidegrees_celsius = (int16_t)(temperature_event.temperature * CENTIDEGREES_PER_DEGREE);
  }

private:
  Adafruit_MPU6050 mpu6050_device;
};

