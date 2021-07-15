#ifndef LIGHTHOUSE_H
#define LIGHTHOUSE_H

#include <Arduino.h>
#include <WebSocketsClient.h>

struct GpioTask {
  unsigned long millis;
  u8 pin;
  u8 val;

  GpioTask(unsigned long _millis, u8 _pin, u8 _val) {
    millis = _millis;
    pin = _pin;
    val = _val;
  }
};

const size_t GPIO_QUEUE_SIZE = 16;

class LighthouseClient {
public:
  LighthouseClient();
  void loop();
  void setup_websocket_client();

private:
  WebSocketsClient websocketClient;
  std::vector<GpioTask> gpioQueue;

  void onEvent(WStype_t type, uint8_t *payload, size_t length);
  void onText(char *payload, size_t length);
};

#endif
