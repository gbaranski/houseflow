#ifndef GPIO_H
#pragma once

#include <Arduino.h>

#include "config.h"

boolean mixingStarted = false;
unsigned long lastMixingMillis = 0;

void startMixing() {
  Serial.println("Starting mix");
  digitalWrite(RELAY_PIN, 0);
  mixingStarted = true;
  lastMixingMillis = millis();
}

#endif