#pragma once

#include <Arduino.h>
#include <EEPROM.h>

#define UUID_LENGTH 36
#define HOST_LENGTH 25
#define OTA_PATH 25
#define DEVICE_TYPE_LENGTH 20

struct DeviceConfig {
  char uid[UUID_LENGTH + 1];
  char secret[UUID_LENGTH + 1];
  char host[HOST_LENGTH + 1];
  char otaPath[OTA_PATH + 1];
  int8_t relayPin;
};

DeviceConfig readDeviceConfig() {
  DeviceConfig deviceConfig;
  EEPROM.get(0, deviceConfig);
  Serial.printf("UID: %s\n", deviceConfig.uid);
  Serial.printf("SECRET: %s\n", deviceConfig.secret);
  Serial.printf("HOST: %s\n", deviceConfig.host);
  Serial.printf("OTA_PATH: %s\n", deviceConfig.otaPath);
  Serial.printf("RELAY_PIN: %u\n", deviceConfig.relayPin);
  return deviceConfig;
}

void writeDeviceConfig(DeviceConfig deviceConfig) {
  EEPROM.put(0, deviceConfig);
  EEPROM.commit();
}