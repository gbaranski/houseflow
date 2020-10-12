#ifndef MEMORY_STORAGE_H
#pragma once

#include <Arduino.h>
#include <EEPROM.h>

#define UUID_LENGTH 36
#define HOST_LENGTH 25
#define OTA_PATH 25

struct ServerConfig {
  char uid[UUID_LENGTH + 1];
  char secret[UUID_LENGTH + 1];
  char host[HOST_LENGTH + 1];
  char ota_path[OTA_PATH + 1];
};

ServerConfig readServerConfig() {
  ServerConfig serverConfig;
  EEPROM.get(0, serverConfig);
  Serial.printf("UID: %s\n", serverConfig.uid);
  Serial.printf("SECRET: %s\n", serverConfig.secret);
  Serial.printf("HOST: %s\n", serverConfig.host);
  Serial.printf("OTA_PATH: %s\n", serverConfig.ota_path);
  return serverConfig;
}

void writeServerConfig(ServerConfig serverConfig) {
  EEPROM.put(0, serverConfig);
  EEPROM.commit();
}

#endif