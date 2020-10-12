# Homeflow

![status](https://img.shields.io/badge/status-OK-GREEN)
[![build-status](https://img.shields.io/github/workflow/status/gbaranski/Control-Home/Docker)](https://github.com/gbaranski/Control-Home/actions)
[![website-status](https://img.shields.io/website?down_color=red&down_message=down&up_color=gree&up_message=up&url=https%3A%2F%2Fcontrol.gbaranski.com)](https://control.gbaranski.com)
[![server-status](https://img.shields.io/website?down_color=red&down_message=down&label=server&up_color=gree&up_message=up&url=https%3A%2F%2Fapi.gbaranski.com)](https://api.gbaranski.com)
![github-stars](https://img.shields.io/github/stars/gbaranski/Control-Home?style=social)

Set up, manage, and control your Homeflow devices, that includes connected home products like lights, cameras, thermostats, relays and more – all from the Homeflow app.

The documentation is divided into several sections:

1. [Components of project](#components-of-project)
2. [Communication and data transmission](#communication-and-data-transmission)

## Components of project

- [Backend infrastructure](#backend-infrastructure)
- [Web App](#web-app)
- [Mobile App](#mobile-app)
- [Embedded devices](#embedded-devices)
- [Firestore database](#firestore-database)
- [Firebase FCM](#firestore-database)
- [Project types](#project-types)

### Backend infrastructure

Server and infrastructure recently had a huge refactor. It will soon move from monolythic to microservices, [here](https://github.com/gbaranski/homeflow/issues/78)

1. [MQTT Broker](#mqtt-broker)
2. [Device service](#device-service)
3. [Auth service](#auth-service)
4. [OTA service](#ota-service)

##### MQTT Broker

Broker which I picked is [emqx](https://github.com/emqx/emqx) because it supports webhooks and authentication out of box

##### Device service

It will help MQTT Broker as webhook service, it will change state of devices in firestore.

##### Auth service

Adds authentication on MQTT broker

##### OTA service

Handle updates over the air for embedded devices

### Web App

Web app made using [React](https://github.com/facebook/react) front-end framework, whole code is in Typescript. App has moved from [Material-UI](https://github.com/mui-org/material-ui), to [antd](https://github.com/ant-design/ant-design)

<img src="https://github.com/gbaranski/Control-Home/blob/master/docs/web_app.png" width="450">

### Mobile App

Mobile app made using [Flutter](https://github.com/flutter/flutter), I picked it over React Native.

<img src="https://github.com/gbaranski/homeflow/blob/master/docs/mobile_app.png" width="150">

### Embedded devices

Embedded devices, microcontrollers i used were ESP8266 and ESP32, those are modules with WiFi. I was using Arduino framework, and C++ languague, thought about C and ESP-IDF but it would take me months.

Devices I made

- Alarmclock on ESP32, just an alarmclock, but with LCD and loud siren to wake me up, also measures temperature and etc, not supported..
- Watermixer on ESP8266, switches relay which activates mixing hot and cold water, currently the device which mostly I focus on.
- Watermixer on Raspberry, made in Node.JS, just to test how it would look like, but its too expensive to make those, currently used for tests.

#### Alarmclock

<img src="https://github.com/gbaranski/homeflow/blob/master/docs/alarmclock.jpg" width="150">

### Firestore database

Project is using [Firestore database](https://firebase.google.com/docs/firestore) for storing devices, users and etc

### Firebase FCM

[Firebase cloud messaging](https://firebase.google.com/docs/cloud-messaging), currently used only for [mobile app](#mobile-app), but in future I expect to use it also for [Web App](#web-app)

### Project types

Types for Typescript, helps a lot with planning, and having cohesion between project components.

## Communication and data transmission

Communication is made on MQTT, clients connects to [MQTT Broker](#mqtt-broker) aswell as embedded devices, they send requests between them.

#### Security

- [Embedded devices](#embedded-devices-security)
- [Web/Mobile Client](#webmobile-client)

##### Embedded devices security

Embedded devices when they try to connect to MQTT broker, they send a secret and their uid, [Auth service](#auth-service) checks if thats allowed to connect, if yes it passed.

##### Web/Mobile Client

Web/Mobile clients are authenticated a little bit different, clients can log in via Google/Email and if they do, they can get authenticated by generating JWT token from Firebase and then sending it with websocket handshake.
