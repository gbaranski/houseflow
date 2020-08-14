#include <Arduino.h>

#ifndef HELPERS_H
#define HELPERS_H
#include "helpers.h"
#endif

#ifndef SECRET_H
#define SECRET_H
#include "secret.h"
#endif

#ifndef OTA_H
#define OTA_H
#include "OTA.h"
#endif

#ifndef EXTLCD_H
#define EXTLCD_H
#include "extLcd.h"
#endif

#ifndef SENSOR_H
#define SENSOR_H
#include "sensor.h"
#endif

#ifndef ALARM_H
#define ALARM_H
#include "alarm.h"
#endif
#include <ArduinoJson.h>
#include <HTTPClient.h>
HTTPClient http;

#include <WiFi.h>
#include <WiFiMulti.h>
#include <WiFiClientSecure.h>

#include <WebSocketsClient.h>

WiFiMulti WiFiMulti;
WebSocketsClient webSocket;

void connectWebSocket();
boolean alarmDuringTest = 0;

void sendDataOverWebsocket()
{
    const int capacity = JSON_OBJECT_SIZE(12);
    StaticJsonDocument<capacity> JSON;
    JSON["ok"] = true;
    JSON["responseFor"] = "GET_DATA";
    JSON["data"]["alarmTime"]["hour"] = parseTimeToHour(getAlarmTime()).toInt();
    JSON["data"]["alarmTime"]["minute"] = parseTimeToMinute(getAlarmTime()).toInt();
    JSON["data"]["alarmTime"]["second"] = 0;
    JSON["data"]["alarmState"] = getAlarmStateBoolean();
    JSON["data"]["sensor"]["temperature"] = getDhtTemperature();
    JSON["data"]["sensor"]["humidity"] = getDhtHumidity();
    JSON["data"]["sensor"]["heatIndex"] = getHeatIndex();

    String stringJson;
    serializeJson(JSON, stringJson);

    webSocket.sendTXT(stringJson);
}

void handleMessage(uint8_t payload[], size_t length)
{
    const int capacity = JSON_OBJECT_SIZE(2) + 3 * JSON_OBJECT_SIZE(1) + 3 * JSON_OBJECT_SIZE(1);
    StaticJsonDocument<capacity> reqJSON;
    deserializeJson(reqJSON, payload);
    String reqType = reqJSON["type"];
    if (reqType == "SET_TIME")
    {
        String hour = reqJSON["data"]["hour"];
        String minute = reqJSON["data"]["minute"];

        saveAlarmTime(formatDoubleDigit(hour) + ":" + formatDoubleDigit(minute));
        Serial.println("[WSc] Received SET_TIME");
    }
    else if (reqType == "SET_STATE")
    {
        bool newState = reqJSON["data"]["state"];
        Serial.println("[WSc] Received state: " + newState);
        setAlarmState(newState);
    }
    else if (reqType == "TEST_SIREN")
    {
        alarmDuringTest = 1;
        Serial.println("[WSc] Testing siren");
    }
    else if (reqType == "RESTART")
    {
        Serial.println("[WSc] Rebooting");
        ESP.restart();
    }
    else
    {
        Serial.println("[WSc] Unknown request");
    }
}

void webSocketEvent(WStype_t type, uint8_t *payload, size_t length)
{

    switch (type)
    {
    case WStype_DISCONNECTED:
        Serial.printf("[WSc] Disconnected!\n");
        connectWebSocket();
        break;
    case WStype_CONNECTED:
        Serial.printf("[WSc] Connected to url: %s\n", payload);
        break;
    case WStype_TEXT:
        handleMessage(payload, length);
        break;
    case WStype_BIN:
        Serial.println("[WSc] Not supported BIN");
        break;
    case WStype_ERROR:
        connectWebSocket();
        break;
    case WStype_FRAGMENT_TEXT_START:
    case WStype_FRAGMENT_BIN_START:
    case WStype_FRAGMENT:
    case WStype_FRAGMENT_FIN:
        break;
    case WStype_PING:
        Serial.println("[WSc] Received ping");
        break;
    case WStype_PONG:
        Serial.println("[WSc] Received pong");
        break;
    }
}

String getToken()
{
    // std::unique_ptr<BearSSL::WiFiClientSecure> client(new BearSSL::WiFiClientSecure);
    // client->setFingerprint(fingerprint);
    // http.begin(*client, TOKEN_SERVER_URL);
    http.begin(TOKEN_SERVER_URL);
    http.addHeader("deviceType", "WATERMIXER");
    http.addHeader("uid", ALARMCLOCK_UID);
    http.addHeader("secret", ALARMCLOCK_SECRET);
    http.addHeader("accept", "text/plain");
    int httpCode = http.GET();
    if (httpCode == 200)
    {
        String token = http.getString();
        http.end();
        Serial.println("Success retreiving token");
        Serial.println(token);
        return token;
    }
    else if (httpCode == 401)
    {
        Serial.println("Unauthorized when attempting to retreive token");
        http.end();
        connectWebSocket();
        return "";
    }
    else
    {
        Serial.println("Unhandled error when fetching token CODE: " + httpCode);
        http.end();
        connectWebSocket();
        return "";
    }
}

void setupWebsocket()
{

    Serial.setDebugOutput(true);

    Serial.println();
    Serial.println();
    Serial.println();

    WiFiMulti.addAP(ssid, password);
}
void connectWebSocket()
{
    for (uint8_t t = 200; t > 0; t--)
    {
        Serial.printf("[WS] WAIT FOR CONNECT %d...\n", t);
        delay(10);
    }
    webSocket.setExtraHeaders((
                                  "token: " + getToken() +
                                  "\r\ndevicetype: ALARMCLOCK" +
                                  "\r\nuid: " + ALARMCLOCK_UID +
                                  "\r\nsecret: " + ALARMCLOCK_SECRET)
                                  .c_str());

    webSocket.begin(websockets_server, websockets_port, websockets_path);
    webSocket.onEvent(webSocketEvent);
    webSocket.enableHeartbeat(2000, 2000, 2);
}

boolean isWifiRunning()
{
    return WiFiMulti.run() == WL_CONNECTED;
}

void webSocketLoop()
{
    webSocket.loop();
}

boolean isAlarmDuringTest()
{
    return alarmDuringTest;
}

void setAlarmDuringTest(bool state)
{
    alarmDuringTest = state;
}