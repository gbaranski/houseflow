#include "lighthouse.hpp"
#include "optserial.hpp"
#include <Arduino.h>
#include <ArduinoJson.h>
#include <WebSocketsClient.h>

#define SERVER_PORT 6001
#define RECONNECT_INTERVAL 10000
#define PING_INTERVAL 5000
#define PONG_INTERVAL 5000
#define DISCONNECT_TIMEOUT_COUNT 2

LighthouseClient::LighthouseClient() {
  OptSerial.println("Constructor called");
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

#include <base64.h>

void LighthouseClient::setup_websocket_client() {
  static auto *this_ptr = this;
  auto handler = [](WStype_t type, uint8_t *payload, size_t length) {
    this_ptr->onEvent(type, payload, length);
  };

  String encoded_credentials = base64::encode(DEVICE_ID ":" DEVICE_SECRET);
  String authorization_header = "Authorization: Basic " + encoded_credentials;

  websocketClient.begin(SERVER_HOST, SERVER_PORT, "/lighthouse/ws");
  websocketClient.setExtraHeaders(authorization_header.c_str());
  websocketClient.onEvent(handler);
  websocketClient.setReconnectInterval(RECONNECT_INTERVAL);
  websocketClient.enableHeartbeat(PING_INTERVAL, PONG_INTERVAL,
                                  DISCONNECT_TIMEOUT_COUNT);
}

template <size_t requestDocCapacity, size_t responseDocCapacity>
void LighthouseClient::onExecute(
    const StaticJsonDocument<requestDocCapacity> &requestDoc,
    StaticJsonDocument<responseDocCapacity> &responseDoc) {

  responseDoc["type"] = "execute-response";
  OptSerial.println("[Lighthouse] received Execute frame");
  const char *command_str = requestDoc["command"];

  DeviceCommand command;

  if (strcmp(command_str, "on-off") == 0) {
    command = DeviceCommand::OnOff;
  } else if (strcmp(command_str, "open-close") == 0) {
    command = DeviceCommand::OpenClose;
  } else {
    OptSerial.printf("[Lighthouse] received invalid command: %s\n", command_str);
    return;
  }

  switch (command) {
#ifdef ON_OFF
  case OnOff: {
    bool on = requestDoc["params"]["on"];
    OptSerial.printf("[Lighthouse] setting `on` to `%d`\n", on);

    digitalWrite(ON_OFF_PIN, on);

    responseDoc["status"] = "success";
    responseDoc["state"]["on"] = on;
    break;
  }
#endif
#ifdef OPEN_CLOSE
  case OpenClose: {
    OptSerial.printf("[Lighthouse] toggling OPEN_PIN for %ums\n",
                  OPEN_CLOSE_TOGGLE_DURATION);

    digitalWrite(OPEN_CLOSE_PIN, LOW);
    auto gpioTask =
        GpioTask(millis() + OPEN_CLOSE_TOGGLE_DURATION, OPEN_CLOSE_PIN, HIGH);
    gpioQueue.push_back(gpioTask);

    responseDoc["status"] = "success";
    break;
  }
#endif
  default:
    OptSerial.printf("[Lighthouse] received unknown command: %s\n", command_str);
    responseDoc["status"] = "error";
    responseDoc["error"] = "function-not-supported";
  }
}

template <size_t requestDocCapacity, size_t responseDocCapacity>
void LighthouseClient::onQuery(
    const StaticJsonDocument<requestDocCapacity> &requestDoc,
    StaticJsonDocument<responseDocCapacity> &responseDoc) {
  OptSerial.println("[Lighthouse] received query frame");

  responseDoc["type"] = "state";
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
    OptSerial.print(F("deserializeJson() failed: "));
    OptSerial.println(error.f_str());
    return;
  }

  responseDoc["id"] = requestDoc["id"];

  const char *frame_type = requestDoc["type"];
  if (strcmp(frame_type, "execute") == 0) {
    onExecute(requestDoc, responseDoc);
  } else if (strcmp(frame_type, "query") == 0) {
    onQuery(requestDoc, responseDoc);
  } else {
    OptSerial.printf("[Lighthouse] received unrecognized frame type: %s\n",
                  frame_type);
    return;
  }

  String buf; // TODO: Optimize this by using raw buffers
  serializeJson(responseDoc, buf);
  responseDoc.clear();
  this->websocketClient.sendTXT(buf);

  auto post_process = millis();
  OptSerial.printf("Processing message took %lu ms\n", post_process - pre_process);
}

void LighthouseClient::onEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    OptSerial.printf("[Lighthouse] disconnected\n");
    break;
  case WStype_CONNECTED:
    OptSerial.printf("[Lighthouse] connected to %s\n", payload);
    break;
  case WStype_TEXT:
    OptSerial.printf("[Lighthouse] received text: %s\n", payload);
    this->onText((char *)payload, length);
    break;
  case WStype_BIN:
    OptSerial.printf("[Lighthouse] received binary\n");
    break;
  case WStype_PING:
    OptSerial.printf("[Lighthouse] received ping\n");
    break;
  case WStype_PONG:
    OptSerial.printf("[Lighthouse] received pong\n");
    break;
  case WStype_ERROR:
    OptSerial.printf("[Lighthouse] received error: %s\n", payload);
    break;
  case WStype_FRAGMENT:
    OptSerial.printf("[Lighthouse] received fragment: %s\n", payload);
    break;
  case WStype_FRAGMENT_BIN_START:
    OptSerial.printf("[Lighthouse] received bin_start: %s\n", payload);
    break;
  case WStype_FRAGMENT_TEXT_START:
    OptSerial.printf("[Lighthouse] received text_start: %s\n", payload);
    break;
  case WStype_FRAGMENT_FIN:
    OptSerial.printf("[Lighthouse] received fin: %s\n", payload);
    break;
  }
}
