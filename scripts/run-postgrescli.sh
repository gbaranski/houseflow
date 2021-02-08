#!/bin/sh
docker-compose -f docker-compose.dev.yml exec -u postgres postgres /usr/bin/psql -d houseflow
