#ifndef HF_GPIO_H
#define HF_GPIO_H

#include <stdbool.h>
#include <driver/gpio.h>
#include "hf_types.h"

#define IO_TAG "io"

typedef struct
{
    gpio_num_t onoff_pin;
} IOConfig;

void io_init(void);

DeviceResponseBody io_handle_command(const char *cmd, DeviceRequestBody *req);

#endif