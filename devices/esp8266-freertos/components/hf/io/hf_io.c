#include "hf_io.h"

#include <esp_err.h>
#include <string.h>
#include <esp_log.h>
#include "sdkconfig.h"

#include "hf_types.h"

static DeviceState g_state = {.on = 0};

void io_init()
{
  #if CONFIG_DEVICE_TRAIT_ONOFF == 1
    #if CONFIG_DEVICE_TRAIT_ONOFF_GPIO == 0
      #error "OnOff GPIO is invalid or is not defined"
    #endif
    #if CONFIG_DEVICE_TRAIT_ONOFF_EXECUTE == 1
      ESP_ERROR_CHECK(gpio_set_direction(CONFIG_DEVICE_TRAIT_ONOFF_GPIO, GPIO_MODE_OUTPUT));
    #endif
  #endif
}

// Handles command and writes to
DeviceResponseBody io_handle_command(const char *cmd, DeviceRequestBody *req)
{
    // If everything went okay, just return this struct, otherwise modify
    DeviceResponseBody res = {
        // NULL by default
        .state = req->state,
        .error = NULL,
        .status = "SUCCESS",
    };


#if CONFIG_DEVICE_TRAIT_ONOFF == 1 && CONFIG_DEVICE_TRAIT_ONOFF_EXECUTE == 1
    if (strcmp(cmd, "action.devices.commands.OnOff") == 0)
    {
        gpio_set_level(CONFIG_DEVICE_TRAIT_ONOFF_GPIO, req->state.on);
        g_state.on = req->state.on;
    }
#else
    // Something that will never exucute, just to satisfy the else below
    if(false == true) {}
#endif
    else
    {
        ESP_LOGE(IO_TAG, "invalid cmd %s", cmd);
        res.error = "functionNotSupported";
        res.status = "ERROR";
        res.state = g_state;
    }

    return res;
}

DeviceResponseBody io_handle_fetch_state()
{
    DeviceResponseBody res = {
        .state = g_state,
        .error = NULL,
        .status = "SUCCESS"};
    return res;
}
