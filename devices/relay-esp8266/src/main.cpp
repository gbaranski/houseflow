#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager

#include "config.h"
#include "gpio.h"
#include "mqtt.h"
#include "remoteUpdate.h"
#include "serverConfig.h"

BearSSL::WiFiClientSecure client;
BearSSL::X509List x509(letsencryptauthorityx3_der,
                       letsencryptauthorityx3_der_len);

void fetchCertAuthority() {
  client.setTrustAnchors(&x509);
  reconnect();
}

void setup() {
  Serial.begin(9600);
  Serial.println("Starting!");
  EEPROM.begin(512);

#if SET_CREDENTIALS == true
  ServerConfig newServerConfig = {DEVICE_UID,  DEVICE_SECRET,
                                  DEVICE_HOST, DEVICE_OTA_PATH,
                                  DEVICE_TYPE, DEVICE_RELAY_PIN};
  writeServerConfig(newServerConfig);
  Serial.println("Success writing ServerConfig to EEPROM");
#endif

  ServerConfig serverConfig = readServerConfig();
  relayPin = serverConfig.relayPin;
  setupGpio();
  Serial.println("Initializing WiFi");
  WiFiManager wifiManager;
  wifiManager.autoConnect();
  Serial.println("Connected to WiFi");

  configTime(3 * 3600, 0, "pool.ntp.org");

  startArduinoOta(&serverConfig);
  checkHttpUpdates(&serverConfig, &client);
  initializeMqtt(&serverConfig, &client);
  fetchCertAuthority();
}

unsigned long lastTimePrintedHeap = 0;

void loop() {
  if (WiFi.status() == WL_CONNECTED) {
    if (!pubSubClient->connected()) {
      fetchCertAuthority();
    } else {
      pubSubClient->loop();
    }
  }

  unsigned long now = millis();
  if (relayTriggered)
    if (now - lastRelayTriggeredMillis > 1000) onTimeoutElapsed();

  arduinoOtaLoop();
  if (now - lastTimePrintedHeap > 5000) {
    Serial.printf("Free heap: %u\n", ESP.getFreeHeap());
    lastTimePrintedHeap = millis();
  }
}