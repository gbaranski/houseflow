#include "lighthouse.hpp"
#include "maybeserial.hpp"
#include <Arduino.h>
#include <ArduinoOTA.h>
#include <ESP8266WiFi.h>
#include <Hash.h>
#include <WebSocketsClient.h>

static LighthouseClient lighthouseClient;

void setupWifi() {
  WiFi.setSleepMode(WIFI_NONE_SLEEP);
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  int i = 0;
  while (WiFi.status() != WL_CONNECTED) { // Wait for the Wi-Fi to connect
    delay(1000);
    MaybeSerial.print(++i);
    MaybeSerial.print(' ');
  }
  MaybeSerial.println('\n');
  MaybeSerial.println("Connection established!");
  MaybeSerial.print("IP address:\t");
  MaybeSerial.println(
      WiFi.localIP()); // Send the IP address of the ESP8266 to the computer
}

void setupMaybeSerial() {
  MaybeSerial.begin(115200);
  MaybeSerial.println("Starting serial");
}

void setupOTA() {
  ArduinoOTA.setHostname("ESP-" DEVICE_ID);
  ArduinoOTA.setPassword(DEVICE_SECRET);
  ArduinoOTA.onStart([]() { MaybeSerial.printf("[OTA] Started\n"); });
  ArduinoOTA.onError([](auto error) {
    MaybeSerial.printf("[OTA] Error (%u)\n", error);

    if (error == OTA_AUTH_ERROR)
      MaybeSerial.println("[OTA] Auth Failed");
    else if (error == OTA_BEGIN_ERROR)
      MaybeSerial.println("[OTA] Begin Failed");
    else if (error == OTA_CONNECT_ERROR)
      MaybeSerial.println("[OTA] Connect Failed");
    else if (error == OTA_RECEIVE_ERROR)
      MaybeSerial.println("[OTA] Receive Failed");
    else if (error == OTA_END_ERROR)
      MaybeSerial.println("[OTA] End Failed");
  });
  ArduinoOTA.onEnd([]() { MaybeSerial.printf("[OTA] End\n"); });
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
  setupMaybeSerial();
  setupWifi();
  setupOTA();
  lighthouseClient.setup_websocket_client();
}

void loop() {
  lighthouseClient.loop();
  ArduinoOTA.handle();
}
