#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
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

void setup() {
  Serial.begin(9600);
  Serial.println("Initializing WiFi");

  WiFi.begin(DEVICE_WIFI_SSID, DEVICE_WIFI_PASSWORD);  // Connect to the network

  Serial.println("Connected to WiFi");
  Serial.print("Connecting to ");
  Serial.print(DEVICE_WIFI_SSID);
  Serial.println(" ...");

  int i = 0;
  while (WiFi.status() != WL_CONNECTED) {  // Wait for the Wi-Fi to connect
    delay(1000);
    Serial.print(++i);
    Serial.print(' ');
  }

  Serial.println('\n');
  Serial.println("Connection established!");
  Serial.print("IP address:\t");
  Serial.println(WiFi.localIP());

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