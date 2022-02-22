#ifndef HOUSEFLOW_HIVE_HPP
#define HOUSEFLOW_HIVE_HPP

#include <Arduino.h>
#include <ArduinoJson.h>
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

class HiveClient {
public:
  static void loop();
  static void init();

private:
  static WebSocketsClient websocketClient;
  static std::vector<GpioTask> gpioQueue;

  static void onEvent(WStype_t type, uint8_t *payload, size_t length);
  static void onText(char *payload, size_t length);

  template <size_t requestDocCapacity, size_t responseDocCapacity>
  static void onReadCharacteristic(const StaticJsonDocument<requestDocCapacity> &requestDoc,
                 StaticJsonDocument<responseDocCapacity> &responseDoc);

  template <size_t requestDocCapacity, size_t responseDocCapacity>
  static void onWriteCharacteristic(const StaticJsonDocument<requestDocCapacity> &requestDoc,
                 StaticJsonDocument<responseDocCapacity> &responseDoc);
};

enum DeviceCommand {
  OnOff,
  OpenClose,
};

#endif
