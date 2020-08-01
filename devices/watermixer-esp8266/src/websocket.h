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
#include <ESP8266WiFi.h>
#include <ESP8266WiFiMulti.h>
#include <WebSocketsClient.h>
#include <ESP8266HTTPClient.h>

HTTPClient http;
ESP8266WiFiMulti WiFiMulti;
WebSocketsClient webSocket;

void connectWebSocket();

void handleMessage(uint8_t payload[], size_t length)
{
    String payloadString = formatPayloadToString(payload, length);
    if (payloadString == "GET_DATA")
    {
        String espData = R"({"remainingSeconds":)" + String(remainingSeconds) +
                         R"(,"isTimerOn":)" + String(isTimerOn) +
                         "}";
        webSocket.sendTXT(espData);
        Serial.println("[WSc] Received GET_DATA");
    }
    else if (payloadString == "START_MIXING")
    {
        Serial.println("[WSc] Received START_MIXING");
        handleStartMixing();
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
    // std::unique_ptr<BearSSL::WiFiClientSecure> client(new BearSSL::WiFiClientSecure);
    // client->setFingerprint(fingerprint);
    // http.begin(*client, TOKEN_SERVER_URL);
    http.begin(TOKEN_SERVER_URL);
    http.addHeader("device", "WATERMIXER");
    http.addHeader("token", WATERMIXER_TOKEN);
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
        handleOTA();
        delay(10);
    }
    webSocket.setExtraHeaders(("token: " + getToken()).c_str());

    webSocket.beginSSL(websockets_server, websockets_port, "/");
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
