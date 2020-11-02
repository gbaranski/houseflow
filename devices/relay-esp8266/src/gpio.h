#pragma once

#include <Arduino.h>

int8_t outputPin;

#if DEVICE_TOGGLE == true
boolean currentState = false;

#else

boolean relayTriggered = false;
unsigned long lastRelayTriggeredMillis = 0;

#endif

void setupGpio() {
  pinMode(outputPin, OUTPUT);

#if DEVICE_TOGGLE == true
  digitalWrite(outputPin, LOW);
#else
  digitalWrite(outputPin, HIGH);
#endif
}

void onTimeoutElapsed() {
#if DEVICE_TOGGLE == false
  lastRelayTriggeredMillis = false;
  digitalWrite(outputPin, HIGH);
#endif
}

void changeOutputState() {
  Serial.println("Changing output state");
#if DEVICE_TOGGLE == true
  currentState = !currentState;
  digitalWrite(outputPin, currentState);
  Serial.println("Changed output state to" + String(currentState));
#else
  digitalWrite(outputPin, LOW);
  relayTriggered = true;
  lastRelayTriggeredMillis = millis();
#endif
}