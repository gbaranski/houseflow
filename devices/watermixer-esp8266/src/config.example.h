#include <Arduino.h>

#ifndef CONFIG_H
#pragma once

#define RELAY_PIN 4
#define MQTT_SERVER "IP"
#define DEVICE_UID "UID"
#define DEVICE_SECRET "SECRET"
#define ON_CONNECT_TOPIC "on/connected"
#define UPDATE_URL "http://IP:PORT/ota/esp8266"

const String START_MIX_TOPIC_REQUEST = DEVICE_UID "/event/startmix/request";
const String START_MIX_TOPIC_RESPONSE = DEVICE_UID "/event/startmix/response";
const char* ON_CONNECT_JSON =
    "{\"uid\": \"" DEVICE_UID "\",\"secret\":\"" DEVICE_SECRET "\"}";

#define VERSION ON_CONNECT_JSON

#endif