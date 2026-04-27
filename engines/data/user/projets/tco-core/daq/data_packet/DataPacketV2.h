#pragma once
#include <stdint.h>

#pragma pack(push, 1)
struct DataPacketV2 {
  uint8_t  s0, s1;
  uint32_t ms, ts;
  int16_t  ax, ay, az;
  int16_t  gx, gy, gz;
  int16_t  temp;
  int32_t  lat_fix, lon_fix;
  uint8_t  sats, crc;
  uint8_t  pad[6];
};
#pragma pack(pop)

static const uint8_t SYNC0 = 0xAA;
static const uint8_t SYNC1 = 0x55;

