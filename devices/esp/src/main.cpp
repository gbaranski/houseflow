#include "optserial.hpp"
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
    OptSerial.print(++i);
    OptSerial.print(' ');
  }
  OptSerial.println('\n');
  OptSerial.println("Connection established!");
  OptSerial.print("IP address:\t");
  OptSerial.println(
      WiFi.localIP()); // Send the IP address of the ESP8266 to the computer
}


void setupOptSerial() {
  OptSerial.begin(115200);
  OptSerial.println("Starting serial");
}

void setupOTA() {
  ArduinoOTA.setHostname("ESP-" DEVICE_ID);
  ArduinoOTA.setPassword(DEVICE_SECRET);
  ArduinoOTA.onStart([]() { OptSerial.printf("[OTA] Started\n"); });
  ArduinoOTA.onError([](auto error) {
    OptSerial.printf("[OTA] Error (%u)\n", error);

    if (error == OTA_AUTH_ERROR)
      OptSerial.println("[OTA] Auth Failed");
    else if (error == OTA_BEGIN_ERROR)
      OptSerial.println("[OTA] Begin Failed");
    else if (error == OTA_CONNECT_ERROR)
      OptSerial.println("[OTA] Connect Failed");
    else if (error == OTA_RECEIVE_ERROR)
      OptSerial.println("[OTA] Receive Failed");
    else if (error == OTA_END_ERROR)
      OptSerial.println("[OTA] End Failed");
  });
  ArduinoOTA.onEnd([]() { OptSerial.printf("[OTA] End\n"); });
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
  setupOptSerial();
  setupWifi();
  setupOTA();
  lighthouseClient.setup_websocket_client();
}

void loop() {
  lighthouseClient.loop();
  ArduinoOTA.handle();
}
