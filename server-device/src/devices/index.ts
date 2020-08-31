import {
  Device as DeviceType,
  Client,
} from '@gbaranski/types';
import { MqttClient } from 'mqtt';
import { getRequestTopic } from '@/topics';
import { publishDeviceDisconnect, publishDeviceData } from '@/services/redis_pub';

export default abstract class Device {
  public static currentDeviceObjects: Device[] = [];

  public status = false;

  constructor(
    private mqttClient: MqttClient,
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected activeDevice: DeviceType.ActiveDevice,
  ) {
    Device.currentDeviceObjects.push(this);
    this.status = true;
    publishDeviceData(this.activeDevice);
  }

  public requestDevice(request: Client.Request): boolean {

    const requestData = {
      type: request.requestType,
      data: request.data,
    };
    console.log('Sending', requestData, `to ${this.firebaseDevice.uid}`);
    this.mqttClient.publish(getRequestTopic(this.firebaseDevice.uid), 'START_MIXING');

    return true;
  }

  public terminateConnection(reason: string): void {
    console.log("should terminate now");
    console.log(
      `Websocket error ${reason} ${this.firebaseDevice.type} UID: ${this.firebaseDevice.uid}`,
    );
    this.status = false;
    Device.currentDeviceObjects = Device.currentDeviceObjects
      .filter((deviceObj) => deviceObj.firebaseDevice.uid !== this.firebaseDevice.uid);
    publishDeviceDisconnect(this.activeDevice);
  }

}
