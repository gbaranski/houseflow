#ifndef SERIAL_H
#define SERIAL_H

#include <Arduino.h>

#ifdef SERIAL_DISABLED

#define OptSerial NullSerial
static class {
public:
  void begin(...) {}
  void print(...) {}
  void printf(...) {}
  void println(...) {}
} NullSerial;

#else

#define OptSerial Serial

#endif

#endif
