#include "config.hpp"
#include "lighthouse.hpp"
#include <Arduino.h>
#include <ArduinoOTA.h>
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <Hash.h>
#include <WebSocketsClient.h>

static LighthouseClient lighthouseClient;

void setupWifi() {
  ESP8266WiFiMulti WiFiMulti;

  WiFiMulti.addAP(WIFI_SSID, WIFI_PASSWORD);
  Serial.printf("Starting WiFi with SSID: %s\n", WIFI_SSID);
  while (WiFiMulti.run() != WL_CONNECTED) {
    Serial.println("Waiting for WiFi to connect");
    delay(100);
  }
}

void setupSerial() {
  Serial.begin(SERIAL_BAUD);
  Serial.println("Starting serial");
}

void setupOTA() {
  ArduinoOTA.setHostname("ESP-" DEVICE_ID);
  ArduinoOTA.setPassword(DEVICE_PASSWORD);
  ArduinoOTA.onStart([]() { Serial.printf("[OTA] Started\n"); });
  ArduinoOTA.onError([](auto error) {
    Serial.printf("[OTA] Error (%u)\n", error);

    if (error == OTA_AUTH_ERROR)
      Serial.println("[OTA] Auth Failed");
    else if (error == OTA_BEGIN_ERROR)
      Serial.println("[OTA] Begin Failed");
    else if (error == OTA_CONNECT_ERROR)
      Serial.println("[OTA] Connect Failed");
    else if (error == OTA_RECEIVE_ERROR)
      Serial.println("[OTA] Receive Failed");
    else if (error == OTA_END_ERROR)
      Serial.println("[OTA] End Failed");
  });
  ArduinoOTA.begin();
  ArduinoOTA.onEnd([]() { Serial.printf("[OTA] End\n"); });
}

void setupGPIO() {
#ifdef ON_OFF
  pinMode(ON_OFF_PIN, OUTPUT);
#endif
#ifdef OPEN_CLOSE
  pinMode(OPEN_CLOSE_PIN, OUTPUT);
#endif
}

void setup() {
  setupGPIO();
  setupSerial();
  setupWifi();
  setupOTA();
  lighthouseClient.setup_websocket_client();
}

void loop() {
  lighthouseClient.loop();
  ArduinoOTA.handle();
}
