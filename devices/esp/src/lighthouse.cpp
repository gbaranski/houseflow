#include <Arduino.h>
#include <WebSocketsClient.h>
#include "lighthouse.hpp"
#include "config.hpp"

#define AUTHORIZATION_HEADER                                                   \
  "Authorization: Basic " DEVICE_ID ":" DEVICE_PASSWORD

Lighthouse::Lighthouse() {
  Serial.println("Constructor called");
  websocketClient = WebSocketsClient();
}

void Lighthouse::loop() { websocketClient.loop(); }

void Lighthouse::setup_websocket_client() {
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

void Lighthouse::onEvent(WStype_t type, uint8_t *payload, size_t length) {
  switch (type) {
  case WStype_DISCONNECTED:
    Serial.printf("[Lighthouse] disconnected\n");
    break;
  case WStype_CONNECTED:
    Serial.printf("[Lighthouse] connected to %s\n", payload);
    break;
  case WStype_TEXT:
    Serial.printf("[Lighthouse] received text: %s\n", payload);
    break;
  case WStype_BIN:
    Serial.printf("[Lighthouse] received binary, len: %zu\n", length);
    digitalWrite(LED_PIN, HIGH);
    delay(1000);
    digitalWrite(LED_PIN, LOW);
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
