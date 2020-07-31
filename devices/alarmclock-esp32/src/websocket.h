#include <Arduino.h>

#ifndef HELPERS_H
#define HELPERS_H
#include "helpers.h"
#endif

#ifndef CONFIG_H
#define CONFIG_H
#include "config.h"
#endif

#ifndef SECRET_H
#define SECRET_H
#include "secret.h"
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

void handleMessage(uint8_t payload[], size_t length)
{
    String payloadString = formatPayloadToString(payload, length);
    if (payloadString == "GET_DATA")
    {
        String espOutput =
            R"({"currentTime":")" + getCurrentTime() +
            R"(","alarmTime":")" + getAlarmTime() +
            R"(","remainingTime":")" + getFormattedRemainingTime() +
            R"(","alarmState":)" + getAlarmStateBoolean() +
            R"(,"temperature":)" + getDhtTemperature() +
            R"(,"humidity":)" + getDhtHumidity() +
            R"(,"heatIndex":)" + getHeatIndex() +
            "}";
        webSocket.sendTXT(espOutput);
        Serial.println("[WSc] Received GET_DATA");
    }
    else if (payloadString.substring(0, payloadString.length() - 5) == "SET_TIME=")
    {
        String time = payloadString.substring(payloadString.length(), payloadString.length() - 5);
        Serial.println("[WSc] Received time: " + time);
        saveAlarmTime(time);
    }
    else if (payloadString.substring(0, payloadString.length() - 1) == "SET_STATE=")
    {
        String state = payloadString.substring(payloadString.length(), payloadString.length() - 1);
        Serial.println("[WSc] Received state: " + state);
        setAlarmState(state);
    }
    else if (payloadString == "TEST_SIREN")
    {
        alarmDuringTest = 1;
        Serial.println("[WSc] Testing alarm");
    }
    else if (payloadString == "RESTART")
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
        webSocket.sendTXT("Connected");
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
    http.begin(TOKEN_SERVER_URL);
    http.addHeader("device", "ALARMCLOCK");
    http.addHeader("token", ALARMCLOCK_TOKEN);
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
    for (uint8_t t = 2; t > 0; t--)
    {
        Serial.printf("[WS] WAIT FOR CONNECT %d...\n", t);
        delay(1000);
    }
    webSocket.setExtraHeaders(("token: " + getToken()).c_str());

    webSocket.begin(websockets_server, websockets_port, "/");
    webSocket.onEvent(webSocketEvent);
    webSocket.enableHeartbeat(15000, 10000, 2);
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