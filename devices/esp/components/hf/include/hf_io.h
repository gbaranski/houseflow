#ifndef HF_GPIO_H
#define HF_GPIO_H

#include <stdbool.h>
#include <driver/gpio.h>
#include "hf_types.h"

#define IO_TAG "io"

void io_init();

DeviceResponseBody io_handle_command(const char *cmd, DeviceRequestBody *req);
DeviceResponseBody io_handle_fetch_state();

#endif
