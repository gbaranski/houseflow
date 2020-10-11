#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager

#include "gpio.h"
#include "memoryStorage.h"
#include "mqtt.h"
#include "remoteUpdate.h"

BearSSL::WiFiClientSecure client;
BearSSL::X509List x509(letsencryptauthorityx3_der,
                       letsencryptauthorityx3_der_len);

void fetchCertAuthority() {
  client.setTrustAnchors(&x509);
  Serial.printf("Fetch cert authority: %u\n", ESP.getFreeHeap());
  reconnect();
}

void setup() {
  Serial.begin(9600);
  Serial.println("Starting!");
  EEPROM.begin(512);

#ifdef SET_CREDENTIALS
  ServerConfig newServerConfig = {
      DEVICE_UID,
      DEVICE_SECRET,
      DEVICE_HOST,
      DEVICE_OTA_PATH,
  };
  writeServerConfig(newServerConfig);
  Serial.println("Success writing ServerConfig to EEPROM");
#endif

  pinMode(RELAY_PIN, OUTPUT);
  digitalWrite(RELAY_PIN, 1);
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();
  Serial.println("Connected to WiFi");

  ServerConfig serverConfig = readServerConfig();

  configTime(3 * 3600, 0, "pool.ntp.org");

  checkUpdates(serverConfig, client);

  initializeMqtt(serverConfig, &client);
  fetchCertAuthority();
}

void loop() {
  if (WiFi.status() == WL_CONNECTED) {
    if (!pubSubClient->connected()) {
      fetchCertAuthority();
    } else {
      pubSubClient->loop();
    }
  }

  if (mixingStarted) {
    unsigned long now = millis();

    if (now - lastMixingMillis > 1000) {
      mixingStarted = false;
      digitalWrite(RELAY_PIN, 1);
    }
  }
}