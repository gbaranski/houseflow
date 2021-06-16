#include "lighthouse.hpp"
#include <Arduino.h>

ExecuteFrame ExecuteFrame::decode(Iterable iter) {
  auto frame = ExecuteFrame{};

  frame.id = iter.get_u16();
  uint16_t command = iter.get_u16();

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
