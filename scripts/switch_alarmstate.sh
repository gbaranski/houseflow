#!/bin/bash
username=$1
authKey=$2
state=$3
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "username: $username" \
  -H "authKey: $authKey" \
  https://api.gbaranski.com:8230/api/alarmclock/getData
