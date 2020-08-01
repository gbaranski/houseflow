#include <Arduino.h>

const String WATERMIXER_TOKEN = "TOKEN";

// WIFI
const char *ssid = "WIFI_SSID";
const char *password = "WIFI_PASS";

// PROD
const char *TOKEN_SERVER_URL = "http://example.com/api/getToken";
const uint16_t websockets_port = 443;
const char *websockets_server = "example.com"; //server adress and port