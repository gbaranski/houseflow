#pragma once

#include <Arduino.h>

int8_t relayPin;

boolean relayTriggered = false;
unsigned long lastRelayTriggeredMillis = 0;

void setupGpio() {
  pinMode(relayPin, OUTPUT);
  digitalWrite(relayPin, HIGH);
}

void onTimeoutElapsed() {
  relayTriggered = false;
  digitalWrite(relayPin, HIGH);
}

void triggerRelay() {
  Serial.println("Triggering relay");
  digitalWrite(relayPin, LOW);
  relayTriggered = true;
  lastRelayTriggeredMillis = millis();
}