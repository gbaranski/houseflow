#include <Arduino.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

#include "config.h"

#ifndef MQTT_H
#pragma once

WiFiClient espClient;
PubSubClient client(espClient);

long lastMsg = 0;
#define MSG_BUFFER_SIZE (50)
char msg[MSG_BUFFER_SIZE];
int value = 0;

void subscribeTopics() { client.subscribe(START_MIX_TOPIC.c_str()); }
void publishConnected() { client.publish(ON_CONNECT_TOPIC, ON_CONNECT_JSON); }

void callback(char* topic, byte* message, unsigned int length) {
  Serial.println(topic);

  Serial.println();
  if (String(topic) == START_MIX_TOPIC) {
    Serial.println("Received start mixing");
    startMixing();
    return;
  }
  Serial.print(". Message: ");
  String messageTemp;

  for (unsigned int i = 0; i < length; i++) {
    Serial.print((char)message[i]);
    messageTemp += (char)message[i];
  }
  Serial.println();
}

void reconnect() {
  // Loop until we're reconnected
  while (!client.connected()) {
    Serial.print("Attempting MQTT connection...");
    // Attempt to connect
    if (client.connect(DEVICE_UID)) {
      Serial.println("connected");
      subscribeTopics();
      publishConnected();
    } else {
      Serial.print("failed, rc=");
      Serial.print(client.state());
      Serial.println(" try again in 5 seconds");
      // Wait 5 seconds before retrying
      delay(5000);
    }
  }
}

void initializeMqtt() {
  Serial.println("Initializing MQTT");
  client.setServer(MQTT_SERVER, 1883);
  client.setCallback(callback);
  subscribeTopics();
  publishConnected();
}

void mqttLoop() {
  if (!client.connected()) {
    reconnect();
  }
  client.loop();
  // unsigned long now = millis();
  // if (now - lastMsg > 2000) {
  //   lastMsg = now;
  //   ++value;
  //   snprintf(msg, MSG_BUFFER_SIZE, "hello world #%ld", value);
  //   Serial.print("Publish message: ");
  //   Serial.println(msg);
  //   client.publish("outTopic", msg);
  // }
}

#endif