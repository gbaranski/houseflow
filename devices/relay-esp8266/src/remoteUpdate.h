#pragma once

#include <Arduino.h>
#include <ArduinoOTA.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <ESP8266httpUpdate.h>
#include <WiFiClientSecure.h>
#include <WiFiUdp.h>

void update_started() {
  Serial.println("CALLBACK:  HTTP update process started");
}

void update_finished() {
  Serial.println("CALLBACK:  HTTP update process finished");
}

void update_progress(int cur, int total) {
  Serial.printf("CALLBACK:  HTTP update process at %d of %d bytes...\n", cur,
                total);
}

void update_error(int err) {
  Serial.printf("CALLBACK:  HTTP update fatal error code %d\n", err);
}

void checkHttpUpdates(DeviceConfig* deviceConfig,
                      BearSSL::WiFiClientSecure* client) {
  String credentialsJson = "{\"uid\": \"" + String(*deviceConfig->uid) +
                           "\",\"secret\":\"" + String(*deviceConfig->secret) +
                           "\"}";
  String ota_url =
      "https://" + String(*deviceConfig->host) + String(*deviceConfig->otaPath);
  Serial.println("Will attempt OTA to: " + ota_url);
  Serial.println("Will publish credentials: " + credentialsJson);
  t_httpUpdate_return ret;

  ESPhttpUpdate.onStart(update_started);
  ESPhttpUpdate.onEnd(update_finished);
  ESPhttpUpdate.onProgress(update_progress);
  ESPhttpUpdate.onError(update_error);

  ret = ESPhttpUpdate.update(*client, ota_url, credentialsJson);

  switch (ret) {
    case HTTP_UPDATE_FAILED:
      Serial.printf("HTTP_UPDATE_FAILD Error (%d): %s\n",
                    ESPhttpUpdate.getLastError(),
                    ESPhttpUpdate.getLastErrorString().c_str());
      break;

    case HTTP_UPDATE_NO_UPDATES:
      Serial.println("HTTP_UPDATE_NO_UPDATES");
      break;

    case HTTP_UPDATE_OK:
      Serial.println("HTTP_UPDATE_OK");
      break;
  }
}

void startArduinoOta(DeviceConfig* deviceConfig) {
  ArduinoOTA.setHostname("ESP8266_" DEVICE_TYPE);
  ArduinoOTA.setPassword(deviceConfig->secret);

  ArduinoOTA.onStart([]() { Serial.println("Start"); });
  ArduinoOTA.onEnd([]() { Serial.println("\nEnd"); });
  ArduinoOTA.onProgress([](unsigned int progress, unsigned int total) {
    Serial.printf("Progress: %u%%\r", (progress / (total / 100)));
  });
  ArduinoOTA.onError([](ota_error_t error) {
    Serial.printf("Error[%u]: ", error);
    if (error == OTA_AUTH_ERROR)
      Serial.println("Auth Failed");
    else if (error == OTA_BEGIN_ERROR)
      Serial.println("Begin Failed");
    else if (error == OTA_CONNECT_ERROR)
      Serial.println("Connect Failed");
    else if (error == OTA_RECEIVE_ERROR)
      Serial.println("Receive Failed");
    else if (error == OTA_END_ERROR)
      Serial.println("End Failed");
  });
  ArduinoOTA.begin();
}

void arduinoOtaLoop() { ArduinoOTA.handle(); }
