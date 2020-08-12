# Control-Home
![code-coverage](https://img.shields.io/badge/coverage-0%25-red)
![status](https://img.shields.io/badge/status-WIP-yellow)
[![build-status](https://img.shields.io/docker/cloud/build/gbaranski19/control-home-api)](https://hub.docker.com/r/gbaranski19/control-home-api)
[![build-size](https://img.shields.io/docker/image-size/gbaranski19/control-home-api?sort=date)](https://hub.docker.com/r/gbaranski19/control-home-api)
[![build-automated](https://img.shields.io/docker/automated/gbaranski19/control-home-api)](https://hub.docker.com/r/gbaranski19/control-home-api)
![github-stars](https://img.shields.io/github/stars/gbaranski/Control-Home?style=social)


Home automation platform for IoT devices like ESP8266 and ESP32, most of it is built on Typescript and C++

## Components of project
* [Node.JS Server](#nodejs-server)
* [Web App](#web-app)
* [Mobile App](#mobile-app)
* [Embedded devices](#embedded-devices)
* [Firestore database](#firestore-database)
* [Firebase FCM](#firestore-database)
* [Project types](#project-types)

### Node.JS Server
Node.JS Server, whole code is in Typescript, main purpose of this server is handling websocket connections incoming from [embedded devices](#embedded-devices) and [Web App](#web-app) and also [Mobile App](#mobile-app). 

### Web App
Web app made using [React](https://github.com/facebook/react) front-end framework, whole code is in Typescript. Currently this is the most developed part of project alongisde Node.JS Server. At the moment im using [Material-UI](https://github.com/mui-org/material-ui), but planning to switch to [antd](https://github.com/ant-design/ant-design)

<img src="https://github.com/gbaranski/Control-Home/blob/add-documentation/docs/web_app.png" width="450">


### Mobile App
Mobile app made using [React-Native(https://github.com/facebook/react-native) which is framework for mobile apps which allows to write in Typescript. It needs full refactoring, but thinking about switching to flutter.

<img src="https://github.com/gbaranski/Control-Home/blob/add-documentation/docs/mobile_app.jpg" width="150">

### Embedded devices
Embedded devices, microcontrollers i used were ESP8266 and ESP32, those are modules with WiFi, and they are quite powerfull. I was using Arduino framework, and C++ languague, thought about C and ESP-IDF but it would take me months. 
|Name|Microcontroller|Description|
|---|---|---|
|Alarmclock|ESP32|Just an alarmclock, but with LCD and loud siren to wake me up|
|Watermixer|ESP8266|Switches a relay which activates mixing hot and cold water|

#### Alarmclock
<img src="https://github.com/gbaranski/Control-Home/blob/add-documentation/docs/alarmclock.jpg" width="150">

### Firestore database
Project is using [Firestore database](https://firebase.google.com/docs/firestore) for storing devices, users and etc

### Firebase FCM
[Firebase cloud messaging](https://firebase.google.com/docs/cloud-messaging), currently used only for [mobile app](#mobile-app), but in future I expect to use it also for [Web App](#web-app)

### Project types
Types for Typescript, helps a lot with planning, and having cohesion between project components.


