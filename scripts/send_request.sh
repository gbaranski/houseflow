#!/bin/bash
query=$1
authKey=$2
time=$3
curl -v \
  -X POST \
  -H "Accept: application/json" \
  -H "authKey: $authKey" \
  -H "state: $time" \
  http://localhost:8080$query
