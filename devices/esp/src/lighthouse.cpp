#include "config.h"
#include "lighthouse.h"
#include <WebSocketsClient.h>

Lighthouse::Lighthouse() 
{
  Serial.println("Constructor called");
  ws_client = WebSocketsClient();
}

void Lighthouse::setup_websocket_client() 
{
  ws_client.begin(LIGHTHOUSE_ADDRESS, 8080, "/ws");
  /* ws_client.onEvent(this->onEvent); */
  ws_client.setReconnectInterval(5000);
  ws_client.enableHeartbeat(15000, 3000, 2);
}

void Lighthouse::loop()
{
  Serial.println("Lighthouse loop");
  delay(100);
  ws_client.loop();
}

void Lighthouse::onEvent(WStype_t type, uint8_t *payload, size_t length) 
{
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
