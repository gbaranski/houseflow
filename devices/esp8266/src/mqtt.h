#pragma once

#include <Arduino.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>  //https://github.com/esp8266/Arduino
#include <PubSubClient.h>

PubSubClient* pubSubClient;

long lastMsg = 0;
#define MSG_BUFFER_SIZE (50)
char msg[MSG_BUFFER_SIZE];
int value = 0;

struct Topic {
  String request;
  String response;
};

struct MqttTopic {
  Topic topic1;
} mqttTopic;

void subscribeTopics() {
  pubSubClient->subscribe(mqttTopic.topic1.request.c_str());
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
  if (String(topic) == mqttTopic.topic1.request) {
    changeOutputState();
    byte* p = (byte*)malloc(length);
    // Copy the payload to the new buffer
    memcpy(p, message, length);
    pubSubClient->publish(mqttTopic.topic1.response.c_str(), p, length);
    // Free the memory
    free(p);

    return;
  } else {
    Serial.printf("%s is not recognized topic", topic);
  }
}

boolean reconnect() {
  Serial.println("Attempting MQTT connection...");
  String clientId = "device_";
  clientId += String(random(0xffff), HEX);

  if (pubSubClient->connect(clientId.c_str(), DEVICE_UID, DEVICE_SECRET)) {
    Serial.println("Success connecting to MQTT");
    subscribeTopics();
  } else {
    Serial.print("failed, rc=");
    Serial.println(pubSubClient->state());
  }
  return pubSubClient->connected();
}

void initializeMqtt(BearSSL::WiFiClientSecure* wifiClientSecure) {
  mqttTopic.topic1 = {
      DEVICE_UID + String("/action1/request"),
      DEVICE_UID + String("/action1/response"),
  };

  pubSubClient =
      new PubSubClient(DEVICE_MQTT_HOST, 8883, callback, *wifiClientSecure);

  Serial.println("Initializing MQTT");
}
