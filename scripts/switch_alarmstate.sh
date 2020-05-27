#!/bin/bash
username=$1
authKey=$2
state=$3
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "username: $username" \
  -H "authKey: $authKey" \
  -H "state: $state" \
  http://localhost:8080/switchAlarmState
