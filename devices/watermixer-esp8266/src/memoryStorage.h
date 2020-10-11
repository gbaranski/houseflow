#include <Arduino.h>
#include <EEPROM.h>

#ifndef MEMORY_STORAGE_H
#pragma once

#define UUID_LENGTH 36
#define MQTT_HOST_LENGTH 25

struct ServerConfig {
  char uid[UUID_LENGTH + 1];
  char secret[UUID_LENGTH + 1];
  char mqttHost[MQTT_HOST_LENGTH + 1];
};

char generateRandomLetter() { return random(65, 90); }

ServerConfig readServerConfig() {
  ServerConfig serverConfig;
  EEPROM.get(0, serverConfig);
  Serial.printf("UID: %s\n", serverConfig.uid);
  Serial.printf("SECRET: %s\n", serverConfig.secret);
  Serial.printf("MQTTHOST: %s\n", serverConfig.mqttHost);
  return serverConfig;
}

void writeServerConfig(ServerConfig serverConfig) {
  EEPROM.put(0, serverConfig);
  EEPROM.commit();
}

#endif