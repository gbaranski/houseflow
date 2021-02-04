FROM gcc:latest as builder


RUN \
  echo "Updating APT packages" && \ 
  apt update

RUN \
  echo "Installing required packages" && \ 
  apt install -y wget openssl libsodium-dev libpq-dev


WORKDIR /build

RUN \
  echo "Downloading mosquitto source code tarball" && \
  wget https://mosquitto.org/files/source/mosquitto-2.0.6.tar.gz && \ 
  echo "Extracting mosquitto source code tarball" && \
  tar -xzf mosquitto-2.0.6.tar.gz && \
  mv mosquitto-2.0.6 mosquitto 

COPY . .

RUN gcc -I/build/mosquitto/include -I/usr/include/postgresql -fPIC -shared mosquitto_houseflow.c -o /build/mosquitto_houseflow.so -lpq -lsodium

FROM eclipse-mosquitto:2.0.6

RUN \
  apk add libpq libsodium

COPY --from=builder /build/mosquitto_houseflow.so /mosquitto/mosquitto_houseflow.so

COPY mosquitto.conf /mosquitto/config/mosquitto.conf
