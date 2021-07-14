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

void LighthouseClient::loop() { websocketClient.loop(); }

void LighthouseClient::setup_websocket_client() {
  static auto *this_ptr = this;
  auto handler = [](WStype_t type, uint8_t *payload, size_t length) {
    this_ptr->onEvent(type, payload, length);
  };

  websocketClient.begin(LIGHTHOUSE_HOST, LIGHTHOUSE_PORT, LIGHTHOUSE_PATH);
  websocketClient.setExtraHeaders(AUTHORIZATION_HEADER);
  websocketClient.onEvent(handler);
  websocketClient.setReconnectInterval(LIGHTHOUSE_RECONNECT_INTERVAL);
  websocketClient.enableHeartbeat(LIGHTHOUSE_PING_INTERVAL,
                                  LIGHTHOUSE_PONG_INTERVAL,
                                  LIGHTHOUSE_DISCONNECT_TIMEOUT_COUNT);
}

void LighthouseClient::onText(char *text, size_t length) {
  static StaticJsonDocument<1024> reqdoc;
  static StaticJsonDocument<1024> resdoc;

  DeserializationError error = deserializeJson(reqdoc, text);
  if (error) {
    Serial.print(F("deserializeJson() failed: "));
    Serial.println(error.f_str());
    return;
  }

  resdoc["id"] = reqdoc["id"];

  const char* frame_type = reqdoc["type"];
  if (strcmp(frame_type, "Execute") == 0) {
    resdoc["type"] = "ExecuteResponse";
    Serial.println("[Lighthouse] received Execute frame");
    const char* command = reqdoc["command"];
    if (strcmp(command, "OnOff") == 0) {
      bool on = reqdoc["params"]["on"];
      Serial.printf("[Lighthouse] setting `on` to `%d`\n", on);
      digitalWrite(LED_PIN, HIGH);
      delay(100);
      digitalWrite(LED_PIN, LOW);

      resdoc["status"] = "Success";
      resdoc["state"]["on"] = on;
    } else {
      Serial.printf("[Lighthouse] received unknown command: %s\n", command);
      resdoc["status"] = "Error";
      resdoc["error"] = "FunctionNotSupported";
    }
  } else if (strcmp(frame_type, "Query") == 0) {
    Serial.println("[Lighthouse] received Query frame");
    return;
  } else {
    Serial.printf("[Lighthouse] received unrecognized frame type: %s\n", frame_type);
    return;
  }

  
  String buf; // TODO: Optimize this by using raw buffers
  serializeJson(resdoc, buf);
  this->websocketClient.sendTXT(buf);

  // Serial.printf("[Lighthouse] received binary, len: %zu\n", length);
  // Iterable iter(payload, length);
  // uint8_t opcode = iter.get_u8();
  // switch (opcode) {
  // case Frame::Opcode::NoOperation:
  //   break;
  // case Frame::Opcode::Execute: {
  //   auto executeFrame = ExecuteFrame::decode(&iter);
  //   Serial.printf("execute frame ID: %u, command: %x\n", executeFrame.id,
  //                 executeFrame.command);

  //   ExecuteResponseFrame executeResponseFrame(
  //       executeFrame.id, ExecuteResponseFrame::Status::Success,
  //       ExecuteResponseFrame::FunctionNotSupported, (char *)"{}");

  //   Iterable iter(buf, sizeof(buf) / sizeof(buf[0]));
  //   iter.put_u8(Frame::Opcode::ExecuteResponse);
  //   executeResponseFrame.encode(&iter);
  //   websocketClient.sendBIN(buf, iter.position - iter.begin);
  //   digitalWrite(LED_PIN, HIGH);
  //   delay(100);
  //   digitalWrite(LED_PIN, LOW);

  //   break;
  // }
  // default:
  //   Serial.printf("unsupported opcode: %x\n", opcode);
  //   break;
  // }
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
    this->onText((char*)payload, length);
    break;
  case WStype_BIN:
    Serial.printf("[Lighthouse] received binary\n");
    break;
  case WStype_PING:
    Serial.printf("[Lighthouse] received ping\n");
    break;
  case WStype_PONG:
    Serial.printf("[Lighthouse] received ping\n");
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
