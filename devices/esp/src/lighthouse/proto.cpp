#include "lighthouse.hpp"
#include <Arduino.h>

ExecuteFrame ExecuteFrame::decode(Iterable *iter) {
  auto frame = ExecuteFrame{};

  frame.id = iter->get_u16();
  uint16_t command = iter->get_u16();

  printf("command: %u\n", command);
  switch (command) {
  case 0:
    frame.command = ExecuteFrame::Command::NoOperation;
    break;
  case 1:
    frame.command = ExecuteFrame::Command::OnOff;
    break;
  default:
    printf("unexpected execute frame command received: %u\n", command);
  };

  return frame;
}

void ExecuteResponseFrame::encode(Iterable *iter) {
  iter->put_u16(this->id);
  iter->put_u8(static_cast<u8>(this->status));
  iter->put_u16(this->error);
  iter->put_string(this->state);
}
