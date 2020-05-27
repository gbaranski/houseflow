#!/bin/bash
authKey=$1
state=$2
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "authKey: $authKey" \
  -H "state: $state" \
  http://localhost:8080/switchAlarmState
