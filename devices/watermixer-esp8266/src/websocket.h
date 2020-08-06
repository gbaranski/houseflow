#include <Arduino.h>

#ifndef HELPERS_H
#define HELPERS_H
#include "helpers.h"
#endif

#ifndef IO_H
#define IO_H
#include "io.h"
#endif

#ifndef OTA_H
#define OTA_H
#include "OTA.h"
#endif

#ifndef SECRET_H
#define SECRET_H
#include "secret.h"
#endif

// #include <WiFiClientSecureBearSSL.h>
#include <ArduinoJson.h>
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <WebSocketsClient.h>
#include <ESP8266HTTPClient.h>
#include <Hash.h>

HTTPClient http;
ESP8266WiFiMulti WiFiMulti;
WebSocketsClient webSocket;

void connectWebSocket();

void sendDataOverWebsocket()
{
    const int capacity = JSON_OBJECT_SIZE(5);
    StaticJsonDocument<capacity> JSON;
    JSON["ok"] = true;
    JSON["responseFor"] = "GET_DATA";
    JSON["data"]["remainingSeconds"] = remainingSeconds;
    JSON["data"]["isTimerOn"] = isTimerOn;
    String stringJson;
    serializeJson(JSON, stringJson);

    webSocket.sendTXT(stringJson);
}

void handleMessage(uint8_t payload[], size_t length)
{
    const int capacity = JSON_OBJECT_SIZE(2) + 3 * JSON_OBJECT_SIZE(1);
    StaticJsonDocument<capacity> reqJSON;
    deserializeJson(reqJSON, payload);
    String reqType = reqJSON["type"];
    if (reqType == "START_MIXING")
    {

        const int capacity = JSON_OBJECT_SIZE(2);
        StaticJsonDocument<capacity> JSON;
        JSON["ok"] = true;
        JSON["responseFor"] = "START_MIXING";
        String stringJSON;
        serializeJson(JSON, stringJSON);
        webSocket.sendTXT(stringJSON);

        Serial.println("[WSc] Received START_MIXING");
        handleStartMixing();
    }
    else if (reqType == "REBOOT")
    {
        const int capacity = JSON_OBJECT_SIZE(2);
        StaticJsonDocument<capacity> JSON;
        JSON["ok"] = true;
        JSON["responseFor"] = "REBOOT";
        String stringJSON;
        serializeJson(JSON, stringJSON);
        webSocket.sendTXT(stringJSON);
        Serial.println("[WSc] Rebooting");
        ESP.restart();
    }
    else
    {
        const int capacity = JSON_OBJECT_SIZE(2);
        StaticJsonDocument<capacity> JSON;
        JSON["ok"] = false;
        JSON["responseFor"] = "UNKNOWN";
        String stringJSON;
        serializeJson(JSON, stringJSON);
        webSocket.sendTXT(stringJSON);

        Serial.println("Reqtype: " + reqType);
        Serial.println("[WSc] Unknown request: " + reqType);
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
    http.addHeader("uid", WATERMIXER_UID);
    http.addHeader("secret", WATERMIXER_SECRET);
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
        handleOTA();
        delay(10);
    }
    webSocket.setExtraHeaders((
                                  "token: " + getToken() +
                                  "\r\ndevicetype: WATERMIXER" +
                                  "\r\nuid: " + WATERMIXER_UID +
                                  "\r\nsecret: " + WATERMIXER_SECRET)
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
