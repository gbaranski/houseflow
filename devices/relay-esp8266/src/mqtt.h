#pragma once

#include <Arduino.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

#include "serverConfig.h"

PubSubClient* pubSubClient;

ServerConfig* mqttServerConfig;

long lastMsg = 0;
#define MSG_BUFFER_SIZE (50)
char msg[MSG_BUFFER_SIZE];
int value = 0;

struct RelayTopic {
  String request;
  String response;
};

struct MqttTopic {
  RelayTopic relayTopic1;
} mqttTopic;

void subscribeTopics() {
  pubSubClient->subscribe(mqttTopic.relayTopic1.request.c_str());
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
  if (String(topic) == mqttTopic.relayTopic1.request) {
    triggerRelay();
    byte* p = (byte*)malloc(length);
    // Copy the payload to the new buffer
    memcpy(p, message, length);
    pubSubClient->publish(mqttTopic.relayTopic1.response.c_str(), p, length);
    // Free the memory
    free(p);

    return;
  } else {
    Serial.printf("%s is not recognized topic", topic);
  }
}

void reconnect() {
  // Loop until we're reconnected
  while (!pubSubClient->connected()) {
    Serial.print("Attempting MQTT connection...");
    // Attempt to connect
    String clientId = "device_";
    clientId += String(random(0xffff), HEX);

    if (pubSubClient->connect(clientId.c_str(), mqttServerConfig->uid,
                              mqttServerConfig->secret)) {
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

void initializeMqtt(ServerConfig* _serverConfig,
                    BearSSL::WiFiClientSecure* wifiClientSecure) {
  mqttServerConfig = _serverConfig;

  mqttTopic.relayTopic1 = {
      String(mqttServerConfig->uid) + String("/event/relay1/request"),
      String(mqttServerConfig->uid) + String("/event/relay1/response"),
  };

  pubSubClient = new PubSubClient(mqttServerConfig->host, 8883, callback,
                                  *wifiClientSecure);

  Serial.println("Initializing MQTT");
}
