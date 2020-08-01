#include <Arduino.h>

#include "sounds/jingleWin.h"
#include "XT_DAC_Audio.h"
XT_Wav_Class JingleWinSong(JingleWin);
XT_DAC_Audio_Class DacAudio(25, 0);

#ifndef ALARM_H
#define ALARM_H
#include "alarm.h"
#endif

#ifndef EXTLCD_H
#define EXTLCD_H
#include "extLcd.h"
#endif

#ifndef SENSOR_H
#define SENSOR_H
#include "sensor.h"
#endif

#ifndef WEBSOCKET_H
#define WEBSOCKET_H
#include "websocket.h"
#endif

#include "OTA.h"

unsigned long previousMillis = 0; // will store last time LED was updated
const int modePushButton = 14;
const int additionalPushButton = 12;
const int sirenOutput = 23;

void setup()
{

    Serial.begin(9600);
    Serial.setDebugOutput(true);
    pinMode(modePushButton, INPUT_PULLUP);
    pinMode(additionalPushButton, INPUT_PULLUP);
    pinMode(sirenOutput, OUTPUT);
    digitalWrite(sirenOutput, 0);
    if (!setupLcd())

        while (true)
        {
            Serial.println("SSD1306 allocation failed");
            delay(1000);
        }
    setupWebsocket();
    while (!isWifiRunning())
    {
        Serial.println("Waiting for wifi...");
        delay(100);
    }
    setupOta();
    connectWebSocket();
    setupNtp();
    clearLcd();
    // printTextLcd("IP: " + wifiManager.getLocalIp(), 1);
    delay(500);
    // wifiManager.setupServerHandling();
    setupSensors();
}

bool lastModeButtonState = false;
bool lastAdditionalButtonState = false;
bool isAlarmOff = false;
unsigned long millisToStop;

void loop()
{
    webSocketLoop();
    handleOta();

    if (isAlarmDuringTest())
    {
        digitalWrite(sirenOutput, 1);
        DacAudio.FillBuffer();              // Fill the sound buffer with data
        if (JingleWinSong.Playing == false) // if not playing,
            DacAudio.Play(&JingleWinSong);  //                play it, this will cause it to repeat and repeat...
    }

    int modeButtonState = digitalRead(modePushButton);

    if (modeButtonState == LOW && !lastModeButtonState)
    {
        Serial.println("Pressed mode button");
        changeLcdMode();
        delay(200);
        lastModeButtonState = true;
    }
    if (modeButtonState == HIGH)
    {
        lastModeButtonState = false;
    }

    int additionalButtonState = digitalRead(additionalPushButton);
    if (additionalButtonState == LOW && !lastAdditionalButtonState)
    {
        Serial.println("Pressed additional button");
        delay(200);
        lastAdditionalButtonState = true;
        isAlarmOff = true;
    }
    if (additionalButtonState == HIGH)
    {
        lastAdditionalButtonState = false;
    }
    if (getAlarmStateBoolean())
    {
        if (isNowAlarmTime())
        {
            if (!isAlarmOff)
            {
                digitalWrite(sirenOutput, 1);

                DacAudio.FillBuffer();              // Fill the sound buffer with data
                if (JingleWinSong.Playing == false) // if not playing,
                    DacAudio.Play(&JingleWinSong);  //                play it, this will cause it to repeat and repeat...
            }
            else
            {
                digitalWrite(sirenOutput, 0);
            }
        }
        else
        {
            isAlarmOff = false;
            digitalWrite(sirenOutput, 0);
        }
    }

    if (millis() - previousMillis >= 1000)
    {
        previousMillis = millis();
        updateTime();
        refreshLcd();
        if (isAlarmDuringTest())
        {
            setAlarmDuringTest(false);
            digitalWrite(sirenOutput, 0);
        }
    }
}
