#!/bin/bash
username=$1
authKey=$2
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "username: $username" \
  -H "authKey: $authKey" \
  http://localhost:8080/getAlarmclockData
