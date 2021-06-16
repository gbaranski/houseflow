#ifndef LIGHTHOUSE_H
#define LIGHTHOUSE_H

#include <Arduino.h>
#include <WebSocketsClient.h>

class LighthouseClient {
public:
  LighthouseClient();
  void loop();
  void setup_websocket_client();

private:
  void onEvent(WStype_t type, uint8_t *payload, size_t length);
  void onBinary(uint8_t *payload, size_t length);
  WebSocketsClient websocketClient;
};

struct Iterable {
  uint8_t *begin;
  uint8_t *end;
  uint8_t *position;

  Iterable(u8 *begin, size_t size)
      : begin(begin), end(begin + size), position(begin) {}

  uint8_t get_u8() {
    uint8_t result = *position;
    if (position < end - 1) {
      position++;
    }
    return result;
  }

  uint16_t get_u16() {
    uint8_t lsb = get_u8();
    uint8_t msb = get_u8();

    return msb | (lsb << 8);
  }

  void put_u8(u8 v) {
    *position = v;
    position++;
  }

  void put_u16(u16 v) {
    put_u8(v >> 8);
    put_u8(v & 0xFF);
  }

  void put_string(char *s) {
    for (uint32_t i = 0; s[i] != '\0'; i++) {
      put_u8((uint8_t)s[i]);
    }
  }
};

struct Frame {
  enum Opcode {
    NoOperation = 0x00,
    State = 0x01,
    StateCheck = 0x02,
    Execute = 0x03,
    ExecuteResponse = 0x04,
  };
};

struct ExecuteFrame {
  static ExecuteFrame decode(Iterable *iter);

  enum Command { NoOperation = 0x0000, OnOff = 0x0001 };

  u16 id;
  Command command;
  char *params;
};

struct ExecuteResponseFrame {

  enum Status {
    Success = 0x0,
    Error = 0x1,
  };

  enum Error {
    None = 0x0,
    FunctionNotSupported = 0x1,
  };

  u16 id;
  Status status;
  enum Error error;
  char *state;

  void encode(Iterable *iter);

  ExecuteResponseFrame(u16 _id, Status _status, enum Error _error, char *_state) {
    id = _id;
    status = _status;
    error = _error;
    state = _state;
  }
};

#endif
