#!/bin/bash
query=$1
authKey=$2
curl -v  \
  -X POST \
  -H "Accept: application/json" \
  -H "state: $authKey" \
  http://192.168.1.110$query
