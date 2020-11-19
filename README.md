# Houseflow

![status](https://img.shields.io/badge/status-OK-GREEN)
[![website-status](https://img.shields.io/website?down_color=red&down_message=down&up_color=gree&up_message=up&url=https%3A%2F%2Fhouseflow.gbaranski.com)](https://houseflow.gbaranski.com)
![github-stars](https://img.shields.io/github/stars/gbaranski/houseflow?style=social)

Set up, manage, and control your Houseflow devices, that includes connected home products like lights, cameras, thermostats, relays and more – all from the Houseflow app.

The documentation is divided into several sections:

- [Houseflow](#houseflow)
  - [Get started](#get-started)
    - [Backend infrastructure](#backend-infrastructure)
        - [MQTT Broker](#mqtt-broker)
        - [Device service](#device-service)
        - [Webhooks service](#webhooks-service)
        - [Auth service](#auth-service)
        - [OTA service](#ota-service)
    - [Mobile App](#mobile-app)
    - [Web App](#web-app)
    - [Embedded devices](#embedded-devices)
    - [Firestore database](#firestore-database)

## Get started
There will be some steps added soon

### Backend infrastructure

1. [MQTT Broker](#mqtt-broker)
2. [Device service](#device-service)
3. [Auth service](#auth-service)
4. [OTA service](#ota-service)

<img src="/docs/architecture.png">

##### MQTT Broker

Houseflow uses [emqx](https://github.com/emqx/emqx) as MQTT Broker. Devices connect to it aswell as [Device service](#device-service). For authorization [Auth service](#auth-service) is being used. It sends HTTP request to [Webhooks service](#webhooks-sevice) on every connect/disconnect

##### Device service
Used to handle all requests from Web or Mobile app, communicates with them over HTTP, for authorization it is using [Firestore](#firestore-database). Redirects all HTTP requests directly to [MQTT Broker](#mqtt-broker) with specific topic, in conclusion embedded device trigger specific event

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

##### Webhooks service
Listens to EMQX connect/disconnect events, changes device state in [Firestore](#firestore-database)

<img src="https://img.shields.io/badge/Golang---?logo=Go&logoColor=FFFFFF&style=for-the-badge&color=00ADD8">

##### Auth service

Adds device and ACL authorization for MQTT broker, prevents devices from subscribing to topic which isn't intended for them and also from connecting unknown devices.


<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

##### OTA service

~~Handle updates over the air for embedded devices~~. Currently not used. Updates are handled via ArduinoOTA, [related issue](https://github.com/gbaranski/houseflow/issues/128).

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

### Mobile App

Mobile app made using [Flutter](https://github.com/flutter/flutter), I picked it over React Native.

<img src="https://img.shields.io/badge/Dart---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=0175C2">

<br>
<img src="https://github.com/gbaranski/houseflow/blob/master/docs/android_pixel2_dashboard.png" width="150">
<img src="https://github.com/gbaranski/houseflow/blob/master/docs/android_pixel2_device_view.png" width="150">

### Web App

Web app made using [React](https://github.com/facebook/react) front-end library with [Antd Pro v5](https://beta-pro.ant.design/).

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">
<br>
<img src="https://github.com/gbaranski/houseflow/blob/master/docs/web_app.png" width="450">


### Embedded devices

Currently supported devices are ESP8266, also there is a version in Node.JS. For ESP8266 I use Arduino framework and some kind of C++.


<img src="https://img.shields.io/badge/C++---?logo=C%2B%2B&logoColor=FFFFFF&style=for-the-badge&color=00599C">

<br>
<img src="https://github.com/gbaranski/houseflow/blob/master/docs/alarmclock.jpg" width="150">

### Firestore database

Project is using [Firestore database](https://firebase.google.com/docs/firestore) for storing devices, users and etc