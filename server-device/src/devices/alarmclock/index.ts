import Device from '../';
import {
  Device as DeviceType,
  Client,
  Alarmclock
} from '@gbaranski/types';
import { MqttClient } from 'mqtt';

class AlarmclockDevice extends Device<Alarmclock.Data> {
  constructor(
    mqttClient: MqttClient,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<Alarmclock.Data>,
  ) {
    super(mqttClient, firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void { // TODO fix later
    console.log({ message });
  }

  public requestDevice(request: Client.Request) {
    const requestData = {
      type: request.requestType,
      data: request.data,
    };
    console.log('Sending', requestData, `to ${this.firebaseDevice.uid}`);
    // this.mqttClient.publish(getRequestTopic(this.firebaseDevice.uid), request.requestType);
    console.log("Not implemeented sending");

    return true;
  }
}

export default AlarmclockDevice;
