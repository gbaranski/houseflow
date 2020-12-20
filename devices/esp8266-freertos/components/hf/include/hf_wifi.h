//
// Created by gbaranski on 19/12/2020.
//

#ifndef ESP8266_FREERTOS_HF_WIFI_H
#define ESP8266_FREERTOS_HF_WIFI_H

static const char *WIFI_TAG = "wifi station";

void wifi_init_sta(unsigned char *ssid, unsigned char *password);

#endif  // ESP8266_FREERTOS_HF_WIFI_H
