#include "config.hpp"
#include "lighthouse.hpp"
#include <Arduino.h>
#include <ArduinoJson.h>
#include <WebSocketsClient.h>

#define AUTHORIZATION_HEADER                                                   \
  "Authorization: Basic " DEVICE_ID ":" DEVICE_PASSWORD

LighthouseClient::LighthouseClient() {
  Serial.println("Constructor called");
  websocketClient = WebSocketsClient();
}

void LighthouseClient::loop() {
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

void LighthouseClient::setup_websocket_client() {
  static auto *this_ptr = this;
  auto handler = [](WStype_t type, uint8_t *payload, size_t length) {
    this_ptr->onEvent(type, payload, length);
  };

  websocketClient.begin(SERVER_HOST, SERVER_PORT, "/lighthouse/ws");
  websocketClient.setExtraHeaders(AUTHORIZATION_HEADER);
  websocketClient.onEvent(handler);
  websocketClient.setReconnectInterval(RECONNECT_INTERVAL);
  websocketClient.enableHeartbeat(PING_INTERVAL, PONG_INTERVAL,
                                  DISCONNECT_TIMEOUT_COUNT);
}

template <size_t requestDocCapacity, size_t responseDocCapacity>
void LighthouseClient::onExecute(
    const StaticJsonDocument<requestDocCapacity> &requestDoc,
    StaticJsonDocument<responseDocCapacity> &responseDoc) {

  responseDoc["type"] = "ExecuteResponse";
  Serial.println("[Lighthouse] received Execute frame");
  const char *command_str = requestDoc["command"];

  DeviceCommand command;

  if (strcmp(command_str, "OnOff") == 0) {
    command = DeviceCommand::OnOff;
  } else if (strcmp(command_str, "OpenClose") == 0) {
    command = DeviceCommand::OpenClose;
  } else {
    Serial.printf("[Lighthouse] received invalid command: %s\n", command_str);
    return;
  }

  switch (command) {
#ifdef ON_OFF
  case OnOff: {
    bool on = requestDoc["params"]["on"];
    Serial.printf("[Lighthouse] setting `on` to `%d`\n", on);

    digitalWrite(ON_OFF_PIN, on);

    responseDoc["status"] = "Success";
    responseDoc["state"]["on"] = on;
    break;
  }
#endif
#ifdef OPEN_CLOSE
  case OpenClose: {
    Serial.printf("[Lighthouse] toggling OPEN_PIN for %ums\n",
                  OPEN_CLOSE_TOGGLE_DURATION);

    digitalWrite(OPEN_CLOSE_PIN, LOW);
    auto gpioTask =
        GpioTask(millis() + OPEN_CLOSE_TOGGLE_DURATION, OPEN_CLOSE_PIN, HIGH);
    gpioQueue.push_back(gpioTask);

    responseDoc["status"] = "Success";
    break;
  }
#endif
  default:
    Serial.printf("[Lighthouse] received unknown command: %s\n", command_str);
    responseDoc["status"] = "Error";
    responseDoc["error"] = "FunctionNotSupported";
  }
}

template <size_t requestDocCapacity, size_t responseDocCapacity>
void LighthouseClient::onQuery(
    const StaticJsonDocument<requestDocCapacity> &requestDoc,
    StaticJsonDocument<responseDocCapacity> &responseDoc) {
  Serial.println("[Lighthouse] received Query frame");

  responseDoc["type"] = "State";
#ifdef ON_OFF
  responseDoc["state"]["on"] = digitalRead(ON_OFF_PIN);
#endif
#ifdef OPEN_CLOSE
#endif
}

void LighthouseClient::onText(char *text, size_t length) {
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
  if (strcmp(frame_type, "Execute") == 0) {
    onExecute(requestDoc, responseDoc);
  } else if (strcmp(frame_type, "Query") == 0) {
    onQuery(requestDoc, responseDoc);
  } else {
    Serial.printf("[Lighthouse] received unrecognized frame type: %s\n",
                  frame_type);
    return;
  }

  String buf; // TODO: Optimize this by using raw buffers
  serializeJson(responseDoc, buf);
  responseDoc.clear();
  this->websocketClient.sendTXT(buf);

  auto post_process = millis();
  Serial.printf("Processing message took %lu ms\n", post_process - pre_process);
}

void LighthouseClient::onEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    Serial.printf("[Lighthouse] disconnected\n");
    break;
  case WStype_CONNECTED:
    Serial.printf("[Lighthouse] connected to %s\n", payload);
    break;
  case WStype_TEXT:
    Serial.printf("[Lighthouse] received text: %s\n", payload);
    this->onText((char *)payload, length);
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
