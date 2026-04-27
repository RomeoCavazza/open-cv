#pragma once
#include <stdint.h>
#include <stddef.h>

inline uint8_t computeCRC8XOR(const uint8_t* data_buffer, size_t buffer_length_bytes) {
  uint8_t crc_checksum = 0;
  for (size_t byte_index = 0; byte_index < buffer_length_bytes; byte_index++) {
    crc_checksum ^= data_buffer[byte_index];
  }
  return crc_checksum;
}

