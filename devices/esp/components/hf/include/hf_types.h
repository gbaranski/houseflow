#ifndef HF_TYPES_H
#define HF_TYPES_H
#include <driver/gpio.h>
#include <stdbool.h>

#include "sdkconfig.h"

typedef struct
{

  #if CONFIG_DEVICE_TRAIT_ONOFF == 1 && CONFIG_DEVICE_TRAIT_ONOFF_QUERY == 1
    bool on;
  #endif

} DeviceState;

typedef struct
{
  const char *command;
  DeviceState state;
} DeviceRequestBody;

typedef struct
{
  // SUCCESS | ERROR
  const char *status;
  // Present only if status == ERROR
  const char *error;
  DeviceState state;
} DeviceResponseBody;

#endif
