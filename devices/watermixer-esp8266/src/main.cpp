#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager

#include "config.h"
#include "gpio.h"
#include "mqtt.h"
#include "ota.h"

void setup() {
  // put your setup code here, to run once:
  pinMode(RELAY_PIN, OUTPUT);
  digitalWrite(RELAY_PIN, 1);
  Serial.begin(9600);
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();

  Serial.println("Connected to WiFi");
  checkUpdates();
  initializeMqtt();
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