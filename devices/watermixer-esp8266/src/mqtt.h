#include <Arduino.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

#include "config.h"

#ifndef MQTT_H
#pragma once

void initializeMqtt(WiFiClient espClient) {
  Serial.println("Initializing MQTT");
  PubSubClient client(espClient);
}

#endif