#pragma once

#include <Arduino.h>
#include <EEPROM.h>

#define UUID_LENGTH 36
#define HOST_LENGTH 25
#define OTA_PATH 25
#define DEVICE_TYPE_LENGTH 20

struct ServerConfig {
  char uid[UUID_LENGTH + 1];
  char secret[UUID_LENGTH + 1];
  char host[HOST_LENGTH + 1];
  char otaPath[OTA_PATH + 1];
  char deviceType[DEVICE_TYPE_LENGTH + 1];
  int8_t relayPin;
};

ServerConfig readServerConfig() {
  ServerConfig serverConfig;
  EEPROM.get(0, serverConfig);
  Serial.printf("UID: %s\n", serverConfig.uid);
  Serial.printf("SECRET: %s\n", serverConfig.secret);
  Serial.printf("HOST: %s\n", serverConfig.host);
  Serial.printf("OTA_PATH: %s\n", serverConfig.otaPath);
  Serial.printf("DEVICE_TYPE: %s\n", serverConfig.deviceType);
  Serial.printf("RELAY_PIN: %u\n", serverConfig.relayPin);
  return serverConfig;
}

void writeServerConfig(ServerConfig serverConfig) {
  EEPROM.put(0, serverConfig);
  EEPROM.commit();
}