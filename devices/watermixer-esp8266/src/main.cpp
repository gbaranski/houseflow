#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager

#include "config.h"
#include "gpio.h"
#include "memoryStorage.h"
#include "mqtt.h"
#include "remoteUpdate.h"

void setup() {
  Serial.begin(9600);
  Serial.println("Starting!");
  EEPROM.begin(512);

#ifdef SET_CREDENTIALS
  ServerConfig newServerConfig = {
      DEVICE_UID,
      DEVICE_SECRET,
      DEVICE_HOST,
      DEVICE_OTA_PATH,
  };
  writeServerConfig(newServerConfig);
  Serial.println("Success writing ServerConfig to EEPROM");
#endif

  pinMode(RELAY_PIN, OUTPUT);
  digitalWrite(RELAY_PIN, 1);
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();
  Serial.println("Connected to WiFi");
  configTime(3 * 3600, 0, "pool.ntp.org");
  ServerConfig serverConfig = readServerConfig();

  checkUpdates(serverConfig);
  initializeMqtt(serverConfig);
}

void loop() {
  if (mixingStarted) {
    unsigned long now = millis();

    if (now - lastMixingMillis > 1000) {
      mixingStarted = false;
      digitalWrite(RELAY_PIN, 1);
    }
  }
  mqttLoop();
}