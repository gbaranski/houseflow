#include <Arduino.h>
#include <WiFiClient.h>
#include <ESP8266mDNS.h>

#ifndef OTA_H
#define OTA_H
#include "OTA.h"
#endif

#ifndef WEBSOCKET_H
#define WEBSOCKET_H
#include "websocket.h"
#endif

#ifndef IO_H
#define IO_H
#include "io.h"
#endif

void setup()
{
  Serial.begin(115200);
  Serial.println("Booting");
  setupGPIO();
  setupWebsocket();
  while (!isWifiRunning())
  {
    Serial.println("Waiting for wifi...");
    delay(100);
  }
  setupOTA();
  connectWebSocket();
}

unsigned long previousSendDataMillis = 0;

void loop()
{
  handleOTA();
  webSocketLoop();
  handleTimer();
  if (millis() - previousSendDataMillis >= 500)
  {
    previousSendDataMillis = millis();
    sendDataOverWebsocket();
  }
}