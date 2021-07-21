#include "config.hpp"
#include "lighthouse.hpp"
#include <Arduino.h>
#include <ArduinoOTA.h>
#include <ESP8266WiFi.h>
#include <Hash.h>
#include <WebSocketsClient.h>

static LighthouseClient lighthouseClient;

void setupWifi() {
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  int i = 0;
  while (WiFi.status() != WL_CONNECTED) { // Wait for the Wi-Fi to connect
    delay(1000);
    Serial.print(++i);
    Serial.print(' ');
  }
  Serial.println('\n');
  Serial.println("Connection established!");
  Serial.print("IP address:\t");
  Serial.println(
      WiFi.localIP()); // Send the IP address of the ESP8266 to the computer
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
  ArduinoOTA.onEnd([]() { Serial.printf("[OTA] End\n"); });
  ArduinoOTA.begin();
}

void setupGPIO() {
#ifdef ON_OFF
  pinMode(ON_OFF_PIN, OUTPUT);
#endif
#ifdef OPEN_CLOSE
  pinMode(OPEN_CLOSE_PIN, OUTPUT);
  digitalWrite(OPEN_CLOSE_PIN, HIGH);
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
