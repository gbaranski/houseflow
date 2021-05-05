#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <WebSocketsClient.h>
#include <Hash.h>
#include "lighthouse.h"
#include "config.h"

void setupWifi() 
{
  ESP8266WiFiMulti WiFiMulti;

  WiFiMulti.addAP(WIFI_SSID, WIFI_PASSWORD);
  Serial.printf("Starting WiFi with SSID: %s\n", WIFI_SSID);
  while (WiFiMulti.run() != WL_CONNECTED) {
    Serial.println("Waiting for WiFi to connect");
    delay(100);
  }
}

void setupSerial()
{
  Serial.begin(115200);
  Serial.println("Starting serial");
}

void setup() 
{
  setupSerial();
  setupWifi();
}

void loop() 
{
  static Lighthouse lighthouse;
  lighthouse.loop();
}
