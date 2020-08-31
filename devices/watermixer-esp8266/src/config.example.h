#include <Arduino.h>

#ifndef CONFIG_H
#pragma once

#define RELAY_PIN 4
#define MQTT_SERVER "IP"
#define DEVICE_UID "UID"
#define DEVICE_SECRET "SECRET"
#define ON_CONNECT_TOPIC "on/connected"

const String START_MIX_TOPIC = DEVICE_UID "/event/todevice/startmix";
const char* ON_CONNECT_JSON =
    "{\"uid\": \"" DEVICE_UID "\",\"secret\":\"" DEVICE_SECRET "\"}";

#endif