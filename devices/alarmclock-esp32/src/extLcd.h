//
// Created by Grzegorz Baranski on 02/04/2020.
//

#include <Arduino.h>
#include <SPI.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#ifndef SCREEN_WIDTH
#define SCREEN_WIDTH 128 // OLED display width, in pixels
#endif

#ifndef SCREEN_HEIGHT
#define SCREEN_HEIGHT 32 // OLED display height, in pixels
#endif

#ifndef lcd_address
#define lcd_address 0x3C
#endif

#ifndef OLED_RESET
#define OLED_RESET -1
#endif

#ifndef ALARM_H
#define ALARM_H
#include "alarm.h"
#endif

#ifndef SENSOR_H
#define SENSOR_H
#include "sensor.h"
#endif

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

int lcdMode = 1;

bool setupLcd()
{
    Wire.begin(5, 4);
    if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C))
    {
        Serial.println("SSD1306 allocation failed");
        return false;
    }
    delay(1000);
    display.clearDisplay();
    display.setTextColor(WHITE);
    display.setTextSize(1);
    display.println("Check it out on github.com/gbaranski");
    display.display();
    delay(1000);
    return true;
}

void clearLcd()
{
    display.setTextColor(WHITE);
    display.setCursor(0, 0);
    display.setTextSize(1);
    display.clearDisplay();
}

void printTextLcd(String lcdText, int fontSize)
{
    clearLcd();
    display.setTextSize(fontSize);
    display.print(lcdText);
    display.display();
}

void changeLcdMode()
{
    lcdMode++;
}

int getLcdMode()
{
    return lcdMode;
}

void refreshLcd()
{
    clearLcd();
    switch (lcdMode)
    {
    case 4 ... INT_MAX:
    case 1:
        lcdMode = 1;
        display.setTextSize(1);
        display.println("Current time      " + getAlarmState());
        display.setTextSize(2);
        display.println(getCurrentTime());
        break;
    case 2:
        display.setTextSize(1);
        display.println("Remaining time");
        display.setTextSize(2);
        if (getAlarmStateBoolean())
        {
            display.println(getFormattedRemainingTime());
        }
        else
        {
            display.println("Alarm OFF");
        }

        break;
    case 3:
        display.setTextSize(1);
        display.cp437(true);
        display.println("Alarm time: " + getAlarmTime());
        display.write(167);
        display.println("Temperature: " + String(getDhtTemperature()));
        display.println("Humidity: " + String(getDhtHumidity()) + "%");
        display.println("Heat index:" + String(getHeatIndex()));
        break;
    }
    display.display();
}
