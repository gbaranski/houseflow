#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino

// needed for library
#include <DNSServer.h>
#include <ESP8266WebServer.h>
#include <WiFiManager.h>  //https://github.com/tzapu/WiFiManager
#include <time.h>

#include "config.h"
#include "deviceConfig.h"
#include "gpio.h"
#include "mqtt.h"
#include "remoteUpdate.h"

BearSSL::WiFiClientSecure client;
BearSSL::X509List x509(letsencryptauthorityx3_der,
                       letsencryptauthorityx3_der_len);

long lastReconnectAttempt = 0;

void configModeCallback(WiFiManager *myWiFiManager) {
  Serial.println("Entered config mode");
  Serial.println(WiFi.softAPIP());

  Serial.println(myWiFiManager->getConfigPortalSSID());
}

void setup() {
  Serial.begin(9600);
  Serial.println("Starting!");
  WiFiManager wifiManager;

  wifiManager.setDebugOutput(false);
  wifiManager.setAPCallback(configModeCallback);
  Serial.println("Initializing WiFi");
  wifiManager.autoConnect();
  Serial.println("Connected to WiFi");

  setupGpio();

  configTime(3 * 3600, 0, "pool.ntp.org", "time.nist.gov");
  Serial.println("\nWaiting for time");
  while (!time(nullptr)) {
    Serial.print(".");
    delay(1000);
  }
  time(nullptr);
  Serial.println("Got time!");
  client.setTrustAnchors(&x509);

  startArduinoOta();
  initializeMqtt(&client);
}

unsigned long lastTimePrintedHeap = 0;

void loop() {
  arduinoOtaLoop();
  unsigned long now = millis();
#if DEVICE_TOGGLE == false
  if (relayTriggered)
    if (now - lastRelayTriggeredMillis > 1000) onTimeoutElapsed();
#endif

  if (!pubSubClient->connected()) {
    if (now - lastReconnectAttempt > 5000) {
      lastReconnectAttempt = now;
      if (reconnect()) lastReconnectAttempt = 0;
    }
  } else {
    pubSubClient->loop();
  }

  if (now - lastTimePrintedHeap > 5000) {
    Serial.printf("Free heap: %u\n", ESP.getFreeHeap());
    lastTimePrintedHeap = millis();
  }
}