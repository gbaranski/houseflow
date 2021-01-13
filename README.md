# Houseflow

[![build](https://img.shields.io/github/workflow/status/gbaranski/houseflow/CI)](https://github.com/gbaranski/houseflow/actions?query=workflow%3ACI)
[![commit-weekly](https://img.shields.io/github/commit-activity/w/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![lines-of-code](https://img.shields.io/tokei/lines/github/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![stars](https://img.shields.io/github/stars/gbaranski/houseflow?style=social)](https://github.com/gbaranski/houseflow)


Set up, manage, and control your Houseflow devices, that includes connected home products like lights, cameras, thermostats, relays and more – all from the Houseflow app.

- [Houseflow](#houseflow)
  - [Get started](#get-started)
        - [Set up Firebase](#set-up-firebase)
  - [Documentation](#documentation)
  - [MQTT Broker](#mqtt-broker)
    - [Device service](#device-service)
    - [Webhooks service](#webhooks-service)
    - [Auth service](#auth-service)
    - [OTA service](#ota-service)
    - [Mobile App](#mobile-app)
    - [Web App](#web-app)
    - [Embedded devices](#embedded-devices)
        - [Alarmclock](#alarmclock)
        - [Gate](#gate)
        - [Watermixer](#watermixer)
        - [Lights](#lights)
        - [Currently supported device types](#currently-supported-device-types)
    - [Firestore database](#firestore-database)
  - [Contributing](#contributing)

## Get started

##### Set up Firebase
1. Go to [Firebase Console](https://console.firebase.google.com/) and create new project
2. Add new app to project
   - For Web, just click Add Web App in project settings, copy configuration and paste it into /app_web/services/firebase.ts on the top
   - For Android add new app in project settings, enter fields, Android package name must match with /app_mobile/android/app/build.gradle.android.defaultConfig.applicationId. You can check SHA-1 by using ./gradlew signingReport inside app_mobile/android. Download the config file and paste it into app_mobile/android/app/google-services.json. Check out [Flutterfire Android docs](https://firebase.flutter.dev/docs/installation/ios)
   - For iOS add new app in project settings, refer to [Flutterfire iOS docs](https://firebase.flutter.dev/docs/installation/ios)
3. Build Web App, go to app_web and run `npm install && npm run build`
4. Enable Firebase Functions, Firebase Firestore(in production mode, it will be changed anyway in next step), Firebase Hosting and Firebase Authentication, install Firebase CLI tools and in root of project run `firebase deploy `
5. Create collections called `devices` and `devices-private`, generate **UUID version 4**, in Linux/OSX by using `echo $(uuidgen) | awk '{print tolower($0)}'` or by using [UUID generator](https://www.uuidgenerator.net/version4), afterwards create document inside `devices` collection which should look like this
**IMPORTANT**

| Value    | Description                                                                            | Type     |
| -------- | -------------------------------------------------------------------------------------- | -------- |
| $UUID    | With uuidv4 you previously generated                                                   | -------- |
| data     | Leave it as empty map                                                                  | Map      |
| geoPoint | Fill up them, they're used for checking if client is close to device                   | GeoPoint |
| ip       | Leave it as unknown, it will be filled up when device connects                         | string   |
| status   | Leave it as false, it will be filled up when device connects                           | bool     |
| type     | Set it to one of [currently supported device types](#currently-supported-device-types) | string   |
| uid      | Replace it with uuidv4, IT MUST BE SAME AS DOCUMENT NAME                               | string   |

<br>
<img src="/docs/get_started_firestore1.png" width=200>
<br>

Now create new document at `devices-private` collection with same document ID as previously generated uuidv4, now generate another UUID with this command `echo $(uuidgen) | awk '{print tolower($0)}'`

| Value  | Description                          | Type   |
| ------ | ------------------------------------ | ------ |
| $UUID  | With uuidv4 you previously generated | ------ |
| secret | Set it to second generated UUID      | string |

<br>
<img src="/docs/get_started_firestore2.png" width=200>

6. Add service account from firebase, navigate to Project Settings -> Service Accounts -> Generate new private key. Name it as `firebaseConfig.json` at put at project root.
7. Fill up .env, you can generate JWT_KEY by using `openssl rand -base64 1024`, and filling up with output

| Value               | Description                                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------- |
| DOMAIN              | Domain address that will be used(used for finding letsencrypt cert path), only for non-dev version |
| JWT_KEY             | `openssl rand -base64 1024`                                                                        |
| DEVICE_API_USERNAME | `echo $(uuidgen) | awk '{print tolower($0)}`                                                       |
| DEVICE_API_PASSWORD | Same as above                                                                                      |

8. Start the server using `docker-compose -f docker-compose.dev.yml up --build` or in production mode `docker-compose up --build`, be aware that production mode requires setting `DOMAIN` enviroment variable and creating SSL Certificate

9.  Its time to flash device, go inside `/devices/esp8266` and copy `platformio.example.ini` to `platformio.ini` and update fields. Now install [CLI version of platformio](https://docs.platformio.org/en/latest/core/) or [VSCode extension](https://marketplace.visualstudio.com/items?itemName=platformio.platformio-ide). Connect device and press flash!

10. Give yourself permission to execute. First off open mobile app/web app and register, this will create new document at users collection, go to user document and add new `Map` inside `devices` array, it shoud look like that

| Value   | Description                                            | Type   |
| ------- | ------------------------------------------------------ | ------ |
| uid     | UID of the device we flashed                           | string |
| execute | Allow for example opening the gate                     | bool   |
| read    | Allow reading device data                              | bool   |
| write   | Allow writing to device, inviting other people and etc | bool   |

11. Ready, refresh the website and device should be visible. Report any problems or issues [here](https://github.com/gbaranski/houseflow)

## Documentation

<img src="/docs/architecture.png">

## MQTT Broker

Houseflow uses [emqx](https://github.com/emqx/emqx) as MQTT Broker. Devices connect to it aswell as [Device service](#device-service). For authorization [Auth service](#auth-service) is being used. It sends HTTP request to [Webhooks service](#webhooks-sevice) on every connect/disconnect.

### Device service
Used to handle all requests from Web or Mobile app, communicates with them over HTTP, for authorization it is using [Firestore](#firestore-database). Redirects all HTTP requests directly to [MQTT Broker](#mqtt-broker) with specific topic, in conclusion embedded device trigger specific event

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

### Webhooks service
Listens to EMQX connect/disconnect events, changes device state in [Firestore](#firestore-database)

<img src="https://img.shields.io/badge/Golang---?logo=Go&logoColor=FFFFFF&style=for-the-badge&color=00ADD8">

### Auth service

Adds device and ACL authorization for MQTT broker, prevents devices from subscribing to topic which isn't intended for them and also from connecting unknown devices.


<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

### OTA service

~~Handle updates over the air for embedded devices~~. Currently not used. Updates are handled via ArduinoOTA, [related issue](https://github.com/gbaranski/houseflow/issues/128).

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">

### Mobile App

Mobile app made using [Flutter](https://github.com/flutter/flutter), I picked it over React Native.

<img src="https://img.shields.io/badge/Dart---?logo=dart&logoColor=FFFFFF&style=for-the-badge&color=0175C2">

<br>
<img src="/docs/android_pixel2_dashboard.png" width="150">
<img src="/docs/android_pixel2_device_view.png" width="150">

### Web App

Web app made using [React](https://github.com/facebook/react) front-end library with [Antd Pro v5](https://beta-pro.ant.design/).

<img src="https://img.shields.io/badge/Typescript---?logo=typescript&logoColor=FFFFFF&style=for-the-badge&color=007ACC">
<br>
<img src="/docs/web_app.png" width="450">


### Embedded devices

Currently supported devices are ESP8266, also there is a version in Node.JS. For ESP8266 I use Arduino framework and some kind of C++.


<img src="https://img.shields.io/badge/C++---?logo=C%2B%2B&logoColor=FFFFFF&style=for-the-badge&color=00599C">

##### Alarmclock
Measures temperatures and wake me up.
<br>
<img src="/docs/alarmclock.jpg" width="150">

##### Gate
ESP01 with relay used to open or close gate remotely, closes circuit for 1s and then opens it again

##### Watermixer
ESP8266 Development Board with relay to trigger mixing water, closes circuit for 1s and then opens it again

##### Lights
ESP8266 Wemos D1 board, connected with a TIP31C transistor to turn on/off and adjust intensity of lights

##### Currently supported device types
- [WATERMIXER](#watermixer)
- [GATE](#gate)
- [GARAGE](#gate)
- [LIGHT](#lights)

### Firestore database

Project is using [Firestore database](https://firebase.google.com/docs/firestore) for storing devices, users and etc

## Contributing
Pull requests are welcome
