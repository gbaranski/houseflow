FROM python:3.9.0

RUN mkdir /opt/workspace
WORKDIR /opt/workspace

RUN python3 -c "$(curl -fsSL https://raw.githubusercontent.com/platformio/platformio/develop/scripts/get-platformio.py)"

ENV PATH=$PATH:/root/.platformio/penv/bin 

RUN platformio platform install \ 
    https://github.com/platformio/platform-espressif8266 \
    --with-package framework-arduinoespressif8266

COPY . .

ENV PLATFORMIO_BUILD_DIR=/build
RUN \
    platformio lib install && \
    platformio run \ 
    --silent \ 
    -D DEVICE_UID=$WATERMIXER_UID \
    -D DEVICE_SECRET=$WATERMIXER_SECRET