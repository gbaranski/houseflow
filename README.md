# Homeflow
![code-coverage](https://img.shields.io/badge/coverage-0%25-red)
![status](https://img.shields.io/badge/status-OK-GREEN)
[![build-status](https://img.shields.io/github/workflow/status/gbaranski/Control-Home/Docker)](https://github.com/gbaranski/Control-Home/actions)
[![website-status](https://img.shields.io/website?down_color=red&down_message=down&up_color=gree&up_message=up&url=https%3A%2F%2Fcontrol.gbaranski.com)](https://control.gbaranski.com)
[![server-status](https://img.shields.io/website?down_color=red&down_message=down&label=server&up_color=gree&up_message=up&url=https%3A%2F%2Fapi.gbaranski.com)](https://api.gbaranski.com)
![github-stars](https://img.shields.io/github/stars/gbaranski/Control-Home?style=social)


Home automation platform for IoT devices like ESP8266 and ESP32, most of it is built on Typescript and C++

## Documentation is a little bit outdated, will update soon
## Documentation
The documentation is divided into several sections:

1. [Components of project](#components-of-project)
2. [Communication and data transmission](#communication-and-data-transmission)

## Components of project
* [Backend infrastructure](#backend-infrastructure)
* [Web App](#web-app)
* [Mobile App](#mobile-app)
* [Embedded devices](#embedded-devices)
* [Firestore database](#firestore-database)
* [Firebase FCM](#firestore-database)
* [Project types](#project-types)

### Backend infrastructure
Server and infrastructure recently had a huge refactor. Now the backend is build in microservice style, previosly monolithic. I keep working on scalability and CI/CD. The backend splits into following microservices:
 1. API Server, atm used only for giving tokens to IoT devices. Built using expressJS, Node.JS, Typescript.
 2. Websocket server at client side, handles communication with web and mobile(soon) communication. Built using [ws](https://www.npmjs.com/package/ws) library, Node.JS and typescript.
 3. Websocket server at device side, communicates with IoT devices. Built using same as service above.
 4. Redis, for connecting device and client microservices together, implemented with Pub-Sub pattern.
 5. Docker holding all those things up.

Node.JS Server, whole code is in Typescript, main purpose of this server is handling websocket connections incoming from [embedded devices](#embedded-devices) and [Web App](#web-app) and also [Mobile App](#mobile-app). 

### Web App
Web app made using [React](https://github.com/facebook/react) front-end framework, whole code is in Typescript. Currently this is the most developed part of project alongisde Node.JS Server. App has moved from [Material-UI](https://github.com/mui-org/material-ui), to [antd](https://github.com/ant-design/ant-design)

<img src="https://github.com/gbaranski/Control-Home/blob/master/docs/web_app.png" width="450">


### Mobile App
Mobile app made using [React-Native](https://github.com/facebook/react-native) which is framework for mobile apps which allows to write in Typescript. It needs full refactoring, but thinking about switching to flutter.

<img src="https://github.com/gbaranski/Control-Home/blob/master/docs/mobile_app.jpg" width="150">

### Embedded devices
Embedded devices, microcontrollers i used were ESP8266 and ESP32, those are modules with WiFi, and they are quite powerfull. I was using Arduino framework, and C++ languague, thought about C and ESP-IDF but it would take me months. 
|Name|Microcontroller|Description|
|---|---|---|
|Alarmclock|ESP32|Just an alarmclock, but with LCD and loud siren to wake me up|
|Watermixer|ESP8266|Switches a relay which activates mixing hot and cold water|

#### Alarmclock
<img src="https://github.com/gbaranski/Control-Home/blob/master/docs/alarmclock.jpg" width="150">

### Firestore database
Project is using [Firestore database](https://firebase.google.com/docs/firestore) for storing devices, users and etc

### Firebase FCM
[Firebase cloud messaging](https://firebase.google.com/docs/cloud-messaging), currently used only for [mobile app](#mobile-app), but in future I expect to use it also for [Web App](#web-app)

### Project types
Types for Typescript, helps a lot with planning, and having cohesion between project components.

## Communication and data transmission
Most of the communication and data transmission is done by WebSocket. Previously it was fully on HTTP, but with HTTP there is one problem, I needed two-way communication to properly receive data from devices, and send to them.

|From|To|Type|
|---|---|---|
|Web/Mobile Client|Server|```Client.Request```|
|Server|Web/Mobile Client|```Client.Response```|
|Embedded|Server|`Device.RequestDevice`|
|Server|Embedded|`Device.ResponseDevice`|

#### Security

* [Embedded devices](#embedded-devices-security)
* [Web/Mobile Client](#webmobile-client)


##### Embedded devices security
Devices for the first step make POST HTTP Request to `/api/getToken` with secret key and token, to the server, that returns them signed JWT Token, with this token he sends upgrade HTTP request to server with headers that contains this JWT Token, then server veryfies that and if okay let him go.

##### Web/Mobile Client
Web/Mobile clients are authenticated a little bit different, clients can log in via Google/Email and if they do, they can get authenticated by generating JWT token from Firebase and then sending it with websocket handshake.

#### Websocket Diagram
Here will be diagram soon



