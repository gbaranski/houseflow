#include <Arduino.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

#include "config.h"
#include "memoryStorage.h"

#ifndef MQTT_H
#pragma once

WiFiClient espClient;
PubSubClient client(espClient);

ServerConfig serverConfig;

long lastMsg = 0;
#define MSG_BUFFER_SIZE (50)
char msg[MSG_BUFFER_SIZE];
int value = 0;

struct StartMix {
  String request;
  String response;
};

struct MqttTopic {
  StartMix startMix;
} mqttTopic;

void subscribeTopics() { client.subscribe(mqttTopic.startMix.request.c_str()); }

void callback(char* topic, byte* message, unsigned int length) {
  Serial.printf("Topic: %s\n", topic);

  Serial.print("Message: ");
  String messageTemp;

  for (unsigned int i = 0; i < length; i++) {
    Serial.print((char)message[i]);
    messageTemp += (char)message[i];
  }
  Serial.println();
  if (String(topic) == mqttTopic.startMix.request) {
    startMixing();
    byte* p = (byte*)malloc(length);
    // Copy the payload to the new buffer
    memcpy(p, message, length);
    client.publish(mqttTopic.startMix.response.c_str(), p, length);
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
    if (client.connect(SHORT_UID, serverConfig.uid, serverConfig.secret)) {
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

void initializeMqtt(ServerConfig _serverConfig) {
  serverConfig = _serverConfig;

  mqttTopic.startMix = {
      String(serverConfig.uid) + String("/event/startmix/request"),
      String(serverConfig.uid) + String("/event/startmix/response"),
  };

  Serial.println("Initializing MQTT");
  client.setServer(serverConfig.mqttHost, 1883);
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
