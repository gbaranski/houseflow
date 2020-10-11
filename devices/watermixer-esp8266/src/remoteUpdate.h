#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <ESP8266httpUpdate.h>
#include <WiFiClientSecure.h>

#include "config.h"

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

void checkUpdates(ServerConfig serverConfig) {
  String credentialsJson = "{\"uid\": \"" + String(serverConfig.uid) +
                           "\",\"secret\":\"" + String(serverConfig.secret) +
                           "\"}";
  String ota_url =
      "https://" + String(serverConfig.host) + String(serverConfig.ota_path);
  Serial.println("Will attempt OTA to: " + ota_url);
  Serial.println("Will publish credentials: " + credentialsJson);
  t_httpUpdate_return ret;

  ESPhttpUpdate.onStart(update_started);
  ESPhttpUpdate.onEnd(update_finished);
  ESPhttpUpdate.onProgress(update_progress);
  ESPhttpUpdate.onError(update_error);

  BearSSL::WiFiClientSecure client;

  BearSSL::X509List x509(letsencryptauthorityx3_der,
                         letsencryptauthorityx3_der_len);
  client.setTrustAnchors(&x509);

  ret = ESPhttpUpdate.update(client, ota_url, credentialsJson);

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