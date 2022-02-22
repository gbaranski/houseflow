#include <Arduino.h>
#include <ArduinoOTA.h>
#include <ESP8266WiFi.h>
#include <Hash.h>
#include <WebSocketsClient.h>

#include "utils.hpp"
#include "hive.hpp"

static HiveClient hiveClient;

void setupWifi() {

  WiFi.setSleepMode(WIFI_NONE_SLEEP);
  Serial.printf("Connecting to %s password: %s\n", TOSTRING(WIFI_SSID), TOSTRING(WIFI_PASSWORD));
  WiFi.begin(TOSTRING(WIFI_SSID), TOSTRING(WIFI_PASSWORD));

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
  Serial.begin(115200);
  Serial.println("Starting serial");
}

void setupOTA() {
  ArduinoOTA.setHostname("ESP-" TOSTRING(DEVICE_ID));
  ArduinoOTA.setPassword(TOSTRING(DEVICE_SECRET));
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
  hiveClient.init();
}

void loop() {
  ArduinoOTA.handle();
  hiveClient.loop();
}
