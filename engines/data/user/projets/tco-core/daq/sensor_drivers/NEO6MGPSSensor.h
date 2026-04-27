#pragma once
#include <SoftwareSerial.h>
#include <TinyGPS++.h>
#include <string.h>

class NEO6MGPSSensor {
public:
  NEO6MGPSSensor(int rx_pin, int tx_pin, uint32_t baud_rate) 
    : software_serial_device(rx_pin, tx_pin), 
      gps_baud_rate(baud_rate), 
      nmea_sentence_buffer_index(0), 
      visible_satellite_count_from_gpgsv(0) {
  }

  void begin() {
    software_serial_device.begin(gps_baud_rate);
  }

  void process(uint8_t maximum_characters_to_read = 20) {
    uint8_t characters_read_count = 0;
    while (software_serial_device.available() > 0 && characters_read_count < maximum_characters_to_read) {
      char received_character = software_serial_device.read();
      tinygpsplus_parser.encode(received_character);
      characters_read_count++;
      
      if (received_character == '$') {
        nmea_sentence_buffer_index = 0;
        nmea_sentence_buffer[nmea_sentence_buffer_index++] = received_character;
      } else if (nmea_sentence_buffer_index > 0 && nmea_sentence_buffer_index < 81) {
        nmea_sentence_buffer[nmea_sentence_buffer_index++] = received_character;
        nmea_sentence_buffer[nmea_sentence_buffer_index] = '\0';
        
        if (received_character == '\n' || received_character == '\r') {
          if (strncmp(nmea_sentence_buffer, "$GPGSV", 6) == 0) {
            parseGPGSVSentence();
          }
          nmea_sentence_buffer_index = 0;
        }
      }
    }
  }

  void processExtra(uint8_t maximum_characters_to_read = 10) {
    process(maximum_characters_to_read);
  }

  bool hasLocation() {
    return tinygpsplus_parser.location.isValid();
  }

  int32_t getLatitude() {
    if (tinygpsplus_parser.location.isValid()) {
      return convertRawDegreesToMicrodegrees(tinygpsplus_parser.location.rawLat());
    }
    return 0;
  }

  int32_t getLongitude() {
    if (tinygpsplus_parser.location.isValid()) {
      return convertRawDegreesToMicrodegrees(tinygpsplus_parser.location.rawLng());
    }
    return 0;
  }

  uint8_t getSatellites() {
    uint32_t satellite_count_from_tinygps = tinygpsplus_parser.satellites.value();
    uint8_t result = 0;
    
    if (visible_satellite_count_from_gpgsv > 0 && visible_satellite_count_from_gpgsv <= 20) {
      result = visible_satellite_count_from_gpgsv;
    }
    
    if (satellite_count_from_tinygps > 0 && satellite_count_from_tinygps <= 20) {
      if (satellite_count_from_tinygps > result) {
        result = (uint8_t)satellite_count_from_tinygps;
      }
    }
    
    return result;
  }

private:
  static int32_t convertRawDegreesToMicrodegrees(const RawDegrees& raw_degrees) {
    int32_t microdegrees_value = (int32_t)raw_degrees.deg * 1000000L + (int32_t)(raw_degrees.billionths / 1000UL);
    return raw_degrees.negative ? -microdegrees_value : microdegrees_value;
  }

  void parseGPGSVSentence() {
    char sentence_buffer_copy[82];
    strncpy(sentence_buffer_copy, nmea_sentence_buffer, 81);
    sentence_buffer_copy[81] = '\0';
    
    char* token = strtok(sentence_buffer_copy, ",");
    uint8_t field_index = 0;
    uint8_t total_satellite_count = 0;
    
    while (token != NULL && field_index < 4) {
      token = strtok(NULL, ",");
      field_index++;
      
      if (field_index == 3 && token != NULL) {
        total_satellite_count = atoi(token);
        if (total_satellite_count > 0 && total_satellite_count <= 20) {
          visible_satellite_count_from_gpgsv = total_satellite_count;
        }
        break;
      }
    }
  }

  SoftwareSerial software_serial_device;
  TinyGPSPlus tinygpsplus_parser;
  const uint32_t gps_baud_rate;
  char nmea_sentence_buffer[82];
  uint8_t nmea_sentence_buffer_index;
  uint8_t visible_satellite_count_from_gpgsv;
};

