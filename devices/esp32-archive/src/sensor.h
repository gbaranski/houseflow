//
// Created by Grzegorz Baranski on 10/04/2020.
//

#include <DHT.h>
#include <Adafruit_Sensor.h>

#define DHTPIN 13
#define DHTTYPE DHT11

DHT dht(DHTPIN, DHTTYPE);

void setupSensors()
{
    dht.begin();
}

float getDhtTemperature()
{
    return dht.readTemperature();
}

float getDhtHumidity()
{
    return dht.readHumidity();
}

float getHeatIndex()
{
    return dht.computeHeatIndex(dht.readTemperature(), dht.readHumidity(), false);
}

void refreshDht()
{
    dht.readTemperature();
}
