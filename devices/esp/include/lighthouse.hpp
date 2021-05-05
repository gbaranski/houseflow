#ifndef LIGHTHOUSE_H
#define LIGHTHOUSE_H

#include <Arduino.h>
#include <WebSocketsClient.h>

class Lighthouse {
  public:
    void loop();
    Lighthouse();
    void setup_websocket_client();

  private:
    void onEvent(WStype_t type, uint8_t *payload, size_t length);
    WebSocketsClient ws_client;
};

#endif
