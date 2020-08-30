#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager

#include "config.h"
#include "mqtt.h"

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();

  Serial.println("Connected to WiFi");
  initializeMqtt();
}

void loop() {
  // put your main code here, to run repeatedly:
  mqttLoop();
}