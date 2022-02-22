#include <Arduino.h>
#include <ArduinoJson.h>
#include <WebSocketsClient.h>

#include "hive.hpp"
#include "utils.hpp"

#define SERVER_PORT 6001
#define RECONNECT_INTERVAL 10000
#define PING_INTERVAL 5000
#define PONG_INTERVAL 5000
#define DISCONNECT_TIMEOUT_COUNT 2

WebSocketsClient HiveClient::websocketClient = WebSocketsClient();
std::vector<GpioTask> HiveClient::gpioQueue = std::vector<GpioTask>();

void HiveClient::loop() {
  websocketClient.loop();

  auto now = millis();
  for (auto it = begin(gpioQueue); it != end(gpioQueue); ++it) {
    if (now >= it->millis) {
      digitalWrite(it->pin, it->val);
      gpioQueue.erase(it);
      break;
    }
  }
}

#include <base64.h>

void HiveClient::init() {
  String encoded_credentials = base64::encode(TOSTRING(ACCESSORY_ID) ":" TOSTRING(ACCESSORY_PASSWORD));
  String authorization_header = "Authorization: Basic " + encoded_credentials;

  websocketClient.begin(TOSTRING(HUB_HOST), HUB_PORT, "/provider/hive/websocket");
  websocketClient.setExtraHeaders(authorization_header.c_str());
  websocketClient.onEvent(onEvent);
  websocketClient.setReconnectInterval(RECONNECT_INTERVAL);
  websocketClient.enableHeartbeat(PING_INTERVAL, PONG_INTERVAL,
                                  DISCONNECT_TIMEOUT_COUNT);
}



void HiveClient::onEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    Serial.printf("[Lighthouse] disconnected\n");
    break;
  case WStype_CONNECTED:
    Serial.printf("[Lighthouse] connected to %s\n", payload);
    break;
  case WStype_TEXT:
    Serial.printf("[Lighthouse] received text: %s\n", payload);
    onText((char *)payload, length);
    break;
  case WStype_BIN:
    Serial.printf("[Lighthouse] received binary\n");
    break;
  case WStype_PING:
    Serial.printf("[Lighthouse] received ping\n");
    break;
  case WStype_PONG:
    Serial.printf("[Lighthouse] received pong\n");
    break;
  case WStype_ERROR:
    Serial.printf("[Lighthouse] received error: %s\n", payload);
    break;
  case WStype_FRAGMENT:
    Serial.printf("[Lighthouse] received fragment: %s\n", payload);
    break;
  case WStype_FRAGMENT_BIN_START:
    Serial.printf("[Lighthouse] received bin_start: %s\n", payload);
    break;
  case WStype_FRAGMENT_TEXT_START:
    Serial.printf("[Lighthouse] received text_start: %s\n", payload);
    break;
  case WStype_FRAGMENT_FIN:
    Serial.printf("[Lighthouse] received fin: %s\n", payload);
    break;
  }
}

void HiveClient::onText(char *text, size_t length) {
  static StaticJsonDocument<1024> requestDoc;
  static StaticJsonDocument<1024> responseDoc;

  auto pre_process = millis();

  DeserializationError error = deserializeJson(requestDoc, text);
  if (error) {
    Serial.print(F("deserializeJson() failed: "));
    Serial.println(error.f_str());
    return;
  }

  responseDoc["id"] = requestDoc["id"];

  const char *frame_type = requestDoc["type"];
  if (strcmp(frame_type, "read-characteristic") == 0)
  {
    onReadCharacteristic(requestDoc, responseDoc);
  }
  else if (strcmp(frame_type, "write-characteristic") == 0)
  {
    onWriteCharacteristic(requestDoc, responseDoc);
  }
  else
  {
    Serial.printf("[Hive] received unrecognized frame type: %s\n",
                  frame_type);
    return;
  }

  String buf; // TODO: Optimize this by using raw buffers
  serializeJson(responseDoc, buf);
  responseDoc.clear();
  Serial.printf("response: %s\n", buf.c_str());
  websocketClient.sendTXT(buf);

  auto post_process = millis();
  Serial.printf("Processing message took %lu ms\n", post_process - pre_process);
}

enum ServiceName
{
  TemperatureSensor,
  HumiditySensor,
  GarageDoorOpener,
  Battery,
  OtherService,
};

ServiceName resolveServiceName(String input)
{
  if (input == "temperature-sensor")
  {
    return ServiceName::TemperatureSensor;
  }
  else if (input == "humidity-sensor")
  {
    return ServiceName::HumiditySensor;
  }
  else if (input == "garage-door-opener")
  {
    return ServiceName::GarageDoorOpener;
  }
  else if (input == "battery")
  {
    return ServiceName::Battery;
  }
  else
  {
    return ServiceName::OtherService;
  }
}

enum CharacteristicName
{
  CurrentTemperature,
  CurrentHumidity,
  CurrentDoorState,
  TargetDoorState,
  BatteryLevel,
  ChargingState,
  OtherCharacteristic,
};

CharacteristicName resolveCharacteristicName(String input)
{
  if (input == "current-temperature")
  {
    return CharacteristicName::CurrentTemperature;
  }
  else if (input == "current-humidity")
  {
    return CharacteristicName::CurrentHumidity;
  }
  else if (input == "current-door-state")
  {
    return CharacteristicName::CurrentDoorState;
  }
  else if (input == "target-door-state")
  {
    return CharacteristicName::TargetDoorState;
  }
  else if (input == "battery-level")
  {
    return CharacteristicName::BatteryLevel;
  }
  else if (input == "charging-state")
  {
    return CharacteristicName::ChargingState;
  }
  else
  {
    return CharacteristicName::OtherCharacteristic;
  }
}

static short int openPercent = 100;

template <size_t requestDocCapacity, size_t responseDocCapacity>
void HiveClient::onReadCharacteristic(const StaticJsonDocument<requestDocCapacity> &requestDoc,
                                      StaticJsonDocument<responseDocCapacity> &responseDoc)
{
  responseDoc["type"] = "characteristic-read-result";
  responseDoc["result"]["status"] = "success";
  responseDoc["id"] = requestDoc["id"];

  auto sendError = [&responseDoc](const char *error)
  {
    Serial.printf("[Hive] read-characteristic failed due to %s\n", error);
    responseDoc["result"] = (char*)0;
    responseDoc["result"]["status"] = "error";
    responseDoc["result"]["body"] = error;
  };

  String serviceNameStr = requestDoc["service-name"];
  String characteristicNameStr = requestDoc["characteristic-name"];
  responseDoc["result"]["body"]["name"] = characteristicNameStr;
  Serial.printf("[Hive] received read-characteristic request for service %s and characteristic %s\n", serviceNameStr.c_str(), characteristicNameStr.c_str());
  auto serviceName = resolveServiceName(serviceNameStr);
  auto characteristicName = resolveCharacteristicName(characteristicNameStr);
  if (serviceName == ServiceName::GarageDoorOpener)
  {
    if (characteristicName == CharacteristicName::CurrentDoorState)
    {
      responseDoc["result"]["body"]["open-percent"] = openPercent;
    }
    else if (characteristicName == CharacteristicName::TargetDoorState)
    {
      sendError("characteristic-read-only");
    }
    else
    {
      sendError("characteristic-not-supported");
    }
  }
  else
  {
    sendError("service-not-supported");
  }
}

template <size_t requestDocCapacity, size_t responseDocCapacity>
void HiveClient::onWriteCharacteristic(const StaticJsonDocument<requestDocCapacity> &requestDoc,
                                       StaticJsonDocument<responseDocCapacity> &responseDoc)
{
  responseDoc["type"] = "characteristic-write-result";
  responseDoc["result"]["status"] = "success";
  responseDoc["result"]["body"] = (char*)0;
  responseDoc["id"] = requestDoc["id"];

  auto sendError = [&responseDoc](const char *error)
  {
    Serial.printf("[Hive] write-characteristic failed due to %s\n", error);
    responseDoc["result"]["status"] = "error";
    responseDoc["result"]["body"] = error;
  };

  String serviceNameStr = requestDoc["service-name"];
  String characteristicNameStr = requestDoc["characteristic"]["name"];
  Serial.printf("[Hive] received write-characteristic request for service %s and characteristic %s\n", serviceNameStr.c_str(), characteristicNameStr.c_str());
  auto serviceName = resolveServiceName(serviceNameStr);
  auto characteristicName = resolveCharacteristicName(characteristicNameStr);
  if (serviceName == ServiceName::GarageDoorOpener)
  {
    if (characteristicName == CharacteristicName::CurrentDoorState)
    {
      sendError("characteristic-write-only");
    }
    else if (characteristicName == CharacteristicName::TargetDoorState)
    {
      openPercent = requestDoc["characteristic"]["open-percent"];
    }
    else
    {
      sendError("characteristic-not-supported");
    }
  }
  else
  {
    sendError("service-not-supported");
  }
}
