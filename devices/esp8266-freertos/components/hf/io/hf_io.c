#include "hf_io.h"

#include <esp_err.h>
#include <string.h>
#include <esp_log.h>

#include "hf_types.h"

static DeviceState g_state = {.on = 0};
static IOConfig g_cfg = {.onoff_pin = 5};

void io_init()
{
    gpio_set_direction(g_cfg.onoff_pin, GPIO_MODE_OUTPUT);
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

    if (strcmp(cmd, "action.devices.commands.OnOff") == 0)
    {
        gpio_set_level(g_cfg.onoff_pin, req->state.on);
        g_state.on = req->state.on;
    }
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