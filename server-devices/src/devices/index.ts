import WebSocket from 'ws';
import {
  AnyDeviceData,
  DateTime,
  Device as DeviceType,
} from '@gbaranski/types';
import WatermixerDevice from './watermixer';
import AlarmclockDevice from './alarmclock';
import mongoose from 'mongoose';

export type AnyDeviceObject = WatermixerDevice | AlarmclockDevice;

export default abstract class Device<DeviceData extends AnyDeviceData> {
  private _status = false;

  public abstract handleMessage(message: WebSocket.Data): void;

  protected initInDb(device: DeviceType.ActiveDevice): void {
    const newDevice = new this.databaseModel(device);
    newDevice.save();
  }
  protected async removeFromDb(device: DeviceType.ActiveDevice): Promise<void> {
    await this.databaseModel.deleteOne({ uid: device.uid });
  }

  protected async updateDevice(device: DeviceType.ActiveDevice): Promise<void> {
    await this.databaseModel.updateOne({ uid: device.uid }, device);
  }

  constructor(
    protected ws: WebSocket,
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected activeDevice: DeviceType.ActiveDevice,
    protected databaseModel: mongoose.Model<mongoose.Document>,
  ) {
    this.initInDb(this.activeDevice);
    this._status = true;
  }

  public requestDevice(
    type: DeviceType.RequestType,
    data?: DateTime | boolean,
  ): boolean {
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
      type,
      data,
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
    this.removeFromDb(this.activeDevice);
  }

  set status(status: boolean) {
    this._status = status;
  }

  get status(): boolean {
    return this._status;
  }
}
