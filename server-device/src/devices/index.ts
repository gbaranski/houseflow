import { publishDeviceData, publishDeviceDisconnect } from '@/services/gcloud';
import { getEventTopic } from '@/topics';
import {
  Device as DeviceType,
  Watermixer,
  Alarmclock,
  AnyDeviceData,
} from '@gbaranski/types';
import { mqttClient } from '..';

export default abstract class Device<
  DataType extends Watermixer.Data | Alarmclock.Data | AnyDeviceData
> {
  public static currentDeviceObjects: Device<AnyDeviceData>[] = [];

  public status = false;

  constructor(
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected _activeDevice: DeviceType.ActiveDevice<DataType>,
  ) {
    Device.currentDeviceObjects.push(this);
    this.status = true;
    publishDeviceData(this._activeDevice);
  }

  public static async sendRequest(
    request: DeviceType.RequestDevice,
    deviceObj: Promise<Device<AnyDeviceData>>,
  ) {
    console.log(`Sending ${request.topic.name} to ${request.topic.uid}`);

    mqttClient.publish(
      getEventTopic(request),
      request.data ? JSON.stringify(request.data) : '',
    );
    deviceObj
      .then((deviceObjectResolved) =>
        deviceObjectResolved.handleRequest(request),
      )
      .catch((e) => {
        //FIXME: fix that one below
        console.log(
          `${e} when searching for deviceObject, might be because server restarted but ESP keeps connection`,
        );
      });
  }

  abstract handleRequest(request: DeviceType.RequestDevice): any;

  public terminateConnection(reason: string): void {
    console.log('should terminate now');
    console.log(
      `Error ${reason} ${this.firebaseDevice.type} UID: ${this.firebaseDevice.uid}`,
    );
    this.status = false;
    Device.currentDeviceObjects = Device.currentDeviceObjects.filter(
      (deviceObj) => deviceObj.firebaseDevice.uid !== this.firebaseDevice.uid,
    );
    publishDeviceDisconnect(this._activeDevice);
  }

  get activeDevice(): DeviceType.ActiveDevice<DataType> {
    return this._activeDevice;
  }
}
