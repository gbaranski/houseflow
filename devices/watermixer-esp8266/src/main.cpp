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

#if SET_CREDENTIALS true
  ServerConfig newServerConfig = {
      DEVICE_UID,
      DEVICE_SECRET,
      MQTT_HOST,
  };
  writeServerConfig(serverConfig);
  Serial.println("Success writing ServerConfig to EEPROM");
#endif

  pinMode(RELAY_PIN, OUTPUT);
  digitalWrite(RELAY_PIN, 1);
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();
  Serial.println("Connected to WiFi");
  ServerConfig serverConfig = readServerConfig();

  checkUpdates("{\"uid\": \"" + String(serverConfig.uid) + "\",\"secret\":\"" +
               String(serverConfig.secret) + "\"}");

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
  // put your main code here, to run repeatedly:
  mqttLoop();
}