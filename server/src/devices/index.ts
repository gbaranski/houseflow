import WebSocket from 'ws';
import {
  AnyDeviceData,
  RequestDevice,
  RequestTypes,
  DateTime,
} from '@gbaranski/types';
import { logSocketError } from '@/cli';

export default abstract class Device<DeviceData extends AnyDeviceData> {
  private _status = false;

  constructor(
    protected ws: WebSocket,
    private _deviceData: DeviceData,
    public readonly deviceName: string,
    public readonly deviceUid: string,
  ) {
    this._status = true;
  }

  abstract handleMessage(message: WebSocket.Data): void;

  protected requestDevice: RequestDevice = async (
    type: RequestTypes,
    data?: DateTime | boolean,
  ): Promise<boolean> => {
    if (!this.ws) {
      return false;
    }
    if (!this.ws.OPEN) {
      return false;
    }
    if (!this._status) {
      return false;
    }

    const requestData = {
      type,
      data,
    };
    this.ws.send(JSON.stringify(requestData));

    return true;
  };

  public terminateConnection(reason: string): void {
    this.ws.terminate();
    logSocketError(this.deviceName, this.deviceUid, reason);
  }

  get deviceData(): DeviceData {
    return this._deviceData;
  }

  set deviceData(data: DeviceData) {
    this._deviceData = data;
  }

  set deviceStatus(status: boolean) {
    this._status = status;
  }

  get deviceStatus(): boolean {
    return this._status;
  }
}
