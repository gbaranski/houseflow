#ifndef HF_TYPES_H
#define HF_TYPES_H
#include <driver/gpio.h>
#include <stdbool.h>


typedef struct {
    bool on;
} DeviceState;


typedef struct {
  const char* correlation_data;
  const char* command;
  DeviceState state;
} DeviceRequest;

typedef struct {
  const char* correlation_data;
  // SUCCESS | ERROR
  const char* status;
  // Present only if status == ERROR
  const char* error;
  DeviceState state;
} DeviceResponse;

#endif