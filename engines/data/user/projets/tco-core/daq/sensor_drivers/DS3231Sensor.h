#pragma once
#include <Wire.h>
#include <RTClib.h>

class DS3231Sensor {
public:
  bool begin() {
    return ds3231_rtc_device.begin();
  }

  uint32_t getUnixTime() {
    return ds3231_rtc_device.now().unixtime();
  }

private:
  RTC_DS3231 ds3231_rtc_device;
};

