#include <Arduino.h>

// GPIO
const int RELAYPIN = 4;

// WIFI
const char *ssid = "Nigeria";
const char *password = "hondamsx125";

// PROD
const char *TOKEN_SERVER_URL = "http://api.gbaranski.com/api/getToken";
const char *websockets_server = "api.gbaranski.com"; //server adress and port
const uint16_t websockets_port = 6436;