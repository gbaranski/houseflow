#include <Arduino.h>

#include <ArduinoOTA.h>
#include <ESPmDNS.h>


void setupOta()
{
    ArduinoOTA.setPort(3232);

    ArduinoOTA.setHostname("Alarmclock-esp32");

    ArduinoOTA.setPassword("hondamsx125");

    ArduinoOTA
        .onStart([]() {
        String type;
        if (ArduinoOTA.getCommand() == U_FLASH)
            type = "sketch";
        else
            type = "filesystem";

        Serial.println("Start updating " + type);
            })
        .onEnd([]() {
                Serial.println("\nEnd");
            })
                .onProgress([](unsigned int progress, unsigned int total) {
                Serial.printf("Progress: %u%%\r", (progress / (total / 100)));
                    })
                .onError([](ota_error_t error) {
                        Serial.printf("Error[%u]: ", error);
                        if (error == OTA_AUTH_ERROR)
                            Serial.println("Auth Failed");
                        else if (error == OTA_BEGIN_ERROR)
                            Serial.println("Begin Failed");
                        else if (error == OTA_CONNECT_ERROR)
                            Serial.println("Connect Failed");
                        else if (error == OTA_RECEIVE_ERROR)
                            Serial.println("Receive Failed");
                        else if (error == OTA_END_ERROR)
                            Serial.println("End Failed");
                    });

                    ArduinoOTA.begin();
}
void handleOta()
{
    ArduinoOTA.handle();
}