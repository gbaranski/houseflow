#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <WebSocketsClient.h>
#include <Hash.h>
#include "lighthouse.hpp"
#include "config.hpp"

static LighthouseClient lighthouseClient;

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
  Serial.begin(SERIAL_BAUD);
  Serial.println("Starting serial");
}

void setupGPIO() 
{
#ifdef ON_OFF
    pinMode(ON_OFF_PIN, OUTPUT);
#endif
#ifdef OPEN_CLOSE
    pinMode(OPEN_CLOSE_PIN, OUTPUT);
#endif
}

void setup() 
{
  setupGPIO();
  setupSerial();
  setupWifi();
  lighthouseClient.setup_websocket_client();
}

void loop() 
{
  lighthouseClient.loop();
}
