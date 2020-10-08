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

void subscribeTopics() { client.subscribe(START_MIX_TOPIC_REQUEST.c_str()); }

void callback(char* topic, byte* message, unsigned int length) {
  Serial.printf("Topic: %s\n", topic);

  Serial.print("Message: ");
  String messageTemp;

  for (unsigned int i = 0; i < length; i++) {
    Serial.print((char)message[i]);
    messageTemp += (char)message[i];
  }
  Serial.println();
  if (String(topic) == START_MIX_TOPIC_REQUEST) {
    startMixing();
    byte* p = (byte*)malloc(length);
    // Copy the payload to the new buffer
    memcpy(p, message, length);
    client.publish(START_MIX_TOPIC_RESPONSE.c_str(), p, length);
    // Free the memory
    free(p);

    return;
  } else {
    Serial.printf("%s is not start mixing topic", topic);
  }
}

void reconnect() {
  // Loop until we're reconnected
  while (!client.connected()) {
    Serial.print("Attempting MQTT connection...");
    // Attempt to connect
    if (client.connect("device_watermixer1", DEVICE_UID, DEVICE_SECRET)) {
      Serial.println("connected");
      subscribeTopics();
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
}

void mqttLoop() {
  if (!client.connected()) {
    reconnect();
  }
  client.loop();
}

#endif
