#ifndef LIGHTHOUSE_H
#define LIGHTHOUSE_H

#include <Arduino.h>
#include <WebSocketsClient.h>

const size_t BUFFER_SIZE = 512;

class LighthouseClient {
public:
  LighthouseClient();
  void loop();
  void setup_websocket_client();

private:
  std::array<u8, BUFFER_SIZE> buf;
  void onEvent(WStype_t type, uint8_t *payload, size_t length);
  void onText(char *payload, size_t length);
  WebSocketsClient websocketClient;
};

#endif
