import {
  Device as DeviceType,
  Client,
  Watermixer,
  Alarmclock,
  AnyDeviceData,
} from '@gbaranski/types';
import { MqttClient } from 'mqtt';
import { publishDeviceDisconnect, publishDeviceData } from '@/services/redis_pub';

export default abstract class Device<DataType extends Watermixer.Data | Alarmclock.Data | AnyDeviceData> {
  public static currentDeviceObjects: Device<AnyDeviceData>[] = [];

  public status = false;

  constructor(
    protected mqttClient: MqttClient,
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected activeDevice: DeviceType.ActiveDevice<DataType>,
  ) {
    Device.currentDeviceObjects.push(this);
    this.status = true;
    publishDeviceData(this.activeDevice);
  }

  abstract requestDevice(request: Client.Request): any;

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
