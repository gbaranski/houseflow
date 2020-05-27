#!/bin/bash
authKey=$1
time=$2
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "authKey: $authKey" \
  -H "time: $time" \
  http://localhost:8080/setAlarmTime
