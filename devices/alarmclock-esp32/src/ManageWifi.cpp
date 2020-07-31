// //
// // Created by Grzegorz Baranski on 01/04/2020.
// //

// #include "ManageWifi.h"

// #ifndef ARDUINO_H
// #include <Arduino.h>
// #endif

// #ifndef ALARMCLOCK_ESP_CONFIG_H
// #include "config.h"
// #endif

// #ifndef ALARMCLOCK_ESP_MANAGEEXTLCD_H
// #include "extLcd.h"
// ManageLcd lcdManager;
// #endif

// #ifndef ALARMCLOCK_ESP_MANAGESENSOR_H
// #include "sensor.h"
// ManageSensor sensorManager;
// #endif

// #ifndef ALARMCLOCK_ESP_ALARM_H
// #include "alarm.h"
// ManageTime wifiTimeManager;
// #endif

// #include <WiFi.h>
// #include <WebServer.h>
// WebServer server(serverPort);

// bool alarmDuringTest = false;

// bool ManageWifi::setupWifiConnection()
// {
//     WiFi.begin(ssid, password);
//     if (WiFi.waitForConnectResult() != WL_CONNECTED)
//     {
//         Serial.printf("Wifi failed");
//         return false;
//     }
//     Serial.print("IP Address: ");
//     Serial.println(WiFi.localIP());

//     const char *headerKeys[] = {"time", "state"};
//     size_t headerKeysSize = sizeof(headerKeys) / sizeof(char *);
//     server.collectHeaders(headerKeys, headerKeysSize);

//     server.begin();

//     return true;
// }

// void handle404()
// {
//     server.send(404, "text/plain", "Not found");
// }

// void handleGetESPData()
// {
//     String espOutput =
//         R"({"currentTime":")" + wifiTimeManager.getTime() +
//         R"(","alarmTime":")" + wifiTimeManager.getAlarmTime() +
//         R"(","remainingTime":")" + wifiTimeManager.getFormattedRemainingTime() +
//         R"(","alarmState":)" + wifiTimeManager.getAlarmStateBoolean() +
//         R"(,"temperature":)" + sensorManager.getDhtTemperature() +
//         R"(,"humidity":)" + sensorManager.getDhtHumidity() +
//         R"(,"heatIndex":)" + sensorManager.getHeatIndex() +
//         "}";
//     Serial.println(espOutput);
//     server.send(200, "application/json", espOutput);
// }
// void handleSetAlarm()
// {
//     if (server.method() != HTTP_POST)
//     {
//         server.send(400, "text/plain", "USE POST");
//         return;
//     }
//     if (!server.hasHeader("time"))
//     {
//         server.send(400, "text/plain", "NO HEADER TIME");
//         return;
//     }

//     server.send(200, "text/plain", "TIME SET TO:" + server.header("time"));
//     lcdManager.printTextLcd("New request!\nAlarm is set to " + server.header("time") + "\nFrom IP " + server.client().remoteIP().toString(), 1);
//     wifiTimeManager.saveAlarmTime(server.header("time"));
// }

// void handleSetAlarmState()
// {
//     if (server.method() != HTTP_POST)
//     {
//         server.send(400, "text/plain", "USE POST");
//         return;
//     }
//     if (!server.hasHeader("state"))
//     {
//         server.send(400, "text/plain", "NO HEADER STATE");
//         return;
//     }

//     wifiTimeManager.setAlarmState(server.header("state").toInt());
//     server.send(200, "text/plain", "New state: " + server.header("state"));
// }

// void handleTestAlarm()
// {
//     alarmDuringTest = true;
//     server.send(200, "text/plain", "OK");
// }

// bool ManageWifi::isAlarmDuringTest()
// {
//     return alarmDuringTest;
// }

// void ManageWifi::stopAlarmTest()
// {
//     alarmDuringTest = false;
// }

// void getHeapSize()
// {
//     server.send(200, "text/plain", String(ESP.getHeapSize()));
// }

// void restartESP()
// {
//     ESP.getHeapSize();
//     server.send(200);
//     ESP.restart();
// }

// void ManageWifi::setupServerHandling()
// {
//     server.onNotFound(handle404);
//     server.on("/getESPData", handleGetESPData);
//     server.on("/setAlarm", handleSetAlarm);
//     server.on("/setAlarmState", handleSetAlarmState);
//     server.on("/testAlarm", handleTestAlarm);
//     server.on("/restartESP", restartESP);
//     server.on("/getHeapSize", getHeapSize);
// }

// String ManageWifi::getLocalIp()
// {
//     return WiFi.localIP().toString();
// }

// void ManageWifi::handleServer()
// {
//     server.handleClient();
// }
