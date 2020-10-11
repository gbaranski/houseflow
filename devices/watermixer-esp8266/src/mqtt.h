#include <Arduino.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

#include "config.h"
#include "memoryStorage.h"

#ifndef MQTT_H
#pragma once

PubSubClient* pubSubClient;

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

void subscribeTopics() {
  pubSubClient->subscribe(mqttTopic.startMix.request.c_str());
}

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
    pubSubClient->publish(mqttTopic.startMix.response.c_str(), p, length);
    // Free the memory
    free(p);

    return;
  } else {
    Serial.printf("%s is not start mixing topic", topic);
  }
}

void reconnect() {
  // Loop until we're reconnected
  while (!pubSubClient->connected()) {
    Serial.print("Attempting MQTT connection...");
    // Attempt to connect
    String clientId = "device_";
    clientId += String(random(0xffff), HEX);

    if (pubSubClient->connect(clientId.c_str(), serverConfig.uid,
                              serverConfig.secret)) {
      Serial.println("connected");
      subscribeTopics();
    } else {
      Serial.print("failed, rc=");
      Serial.print(pubSubClient->state());
      Serial.println(" try again in 5 seconds");
      // Wait 5 seconds before retrying
      delay(5000);
    }
  }
}

void initializeMqtt(ServerConfig _serverConfig,
                    BearSSL::WiFiClientSecure* wifiClientSecure) {
  serverConfig = _serverConfig;

  mqttTopic.startMix = {
      String(serverConfig.uid) + String("/event/startmix/request"),
      String(serverConfig.uid) + String("/event/startmix/response"),
  };

  pubSubClient =
      new PubSubClient(serverConfig.host, 8883, callback, *wifiClientSecure);
  // while (true) {
  //   if (!pubSubClient->connected()) {
  //     reconnect();
  //   }
  //   pubSubClient->loop();
  // }

  Serial.println("Initializing MQTT");
}

#endif
