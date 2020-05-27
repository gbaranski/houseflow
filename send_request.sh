#!/bin/bash
authKey=$1
curl --header "Content-Type: application/json" \
  --request POST \
  --data @<(cat <<EOF
  {
    "authKey": "$authKey"
  }
EOF
) \
  -H "Accept: application/json" \
  http://localhost:8080/getAlarmClock
