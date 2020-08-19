import WebSocket from 'ws';
import {
  AnyDeviceData,
  DateTime,
  Device as DeviceType,
  Client,
} from '@gbaranski/types';
import WatermixerDevice from './watermixer';
import AlarmclockDevice from './alarmclock';
import { publishDeviceDisconnect } from '@/services/redis_pub';

export type AnyDeviceObject = WatermixerDevice | AlarmclockDevice;

export default abstract class Device<DeviceData extends AnyDeviceData> {
  public static currentDeviceObjects: AnyDeviceObject[] = [];

  private _status = false;

  public abstract handleMessage(message: WebSocket.Data): void;

  constructor(
    protected ws: WebSocket,
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected activeDevice: DeviceType.ActiveDevice,
  ) {
    Device.currentDeviceObjects.push(this);
    this._status = true;
  }

  public requestDevice(request: Client.Request): boolean {
    if (!this.ws) {
      throw new Error('Websocket is not defined');
    }
    if (!this.ws.OPEN) {
      throw new Error('Websocket is not at OPEN state');
    }
    if (!this._status) {
      throw new Error('Device status is false');
    }
    const requestData = {
      type: request.requestType,
      data: request.data,
    };
    console.log(requestData);
    this.ws.send(JSON.stringify(requestData));

    return true;
  }

  public terminateConnection(reason: string): void {
    this.ws.terminate();
    console.log(
      `Websocket error ${reason} ${this.firebaseDevice.type} UID: ${this.firebaseDevice.uid}`,
    );
    this._status = false;
    publishDeviceDisconnect(this.activeDevice);
  }

  set status(status: boolean) {
    this._status = status;
  }

  get status(): boolean {
    return this._status;
  }
}
