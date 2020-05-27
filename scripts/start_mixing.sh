#!/bin/bash
authKey=$1
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "authKey: $authKey" \
  http://localhost:8080/startMixing