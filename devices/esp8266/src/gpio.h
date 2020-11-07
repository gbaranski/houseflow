#pragma once

#include <Arduino.h>

#if DEVICE_TOGGLE == true
boolean currentState = false;

#else

boolean relayTriggered = false;
unsigned long lastRelayTriggeredMillis = 0;

#endif

void setupGpio() {
  pinMode(DEVICE_OUTPUT_PIN, OUTPUT);

#if DEVICE_TOGGLE == true
  digitalWrite(DEVICE_OUTPUT_PIN, LOW);
#else
  digitalWrite(DEVICE_OUTPUT_PIN, HIGH);
#endif
}

void onTimeoutElapsed() {
#if DEVICE_TOGGLE == false
  lastRelayTriggeredMillis = false;
  digitalWrite(DEVICE_OUTPUT_PIN, HIGH);
#endif
}

void changeOutputState() {
  Serial.println("Changing output state");
#if DEVICE_TOGGLE == true
  currentState = !currentState;
  digitalWrite(DEVICE_OUTPUT_PIN, currentState);
  Serial.println("Changed output state to" + String(currentState));
#else
  digitalWrite(DEVICE_OUTPUT_PIN, LOW);
  relayTriggered = true;
  lastRelayTriggeredMillis = millis();
#endif
}