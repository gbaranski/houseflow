#include <Arduino.h>

#ifndef CONFIG_H
#define CONFIG_H
#include "config.h"
#endif

bool isTimerOn;
unsigned long previousTickMillis = 0;
int remainingSeconds = 0;

void setupGPIO()
{
    pinMode(RELAYPIN, OUTPUT);
    digitalWrite(RELAYPIN, 1);
}

void handleStartMixing()
{
    digitalWrite(RELAYPIN, 0);
    isTimerOn = true;
    remainingSeconds = 600;
}

void handleTimer()
{
    if (remainingSeconds <= 598)
    {
        digitalWrite(RELAYPIN, 1);
    }
    if (isTimerOn)
    {
        if (millis() - previousTickMillis >= 1000)
        {
            previousTickMillis = millis();
            remainingSeconds--;
            Serial.println(remainingSeconds);
            if (remainingSeconds == 0)
            {
                isTimerOn = false;
            }
        }
    }
}

int getRemainingSeconds()
{
    return remainingSeconds;
}
bool getIsTimerOn()
{
    return isTimerOn;
}