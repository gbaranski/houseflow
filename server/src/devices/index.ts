import WebSocket from 'ws';
import {
  AnyDeviceData,
  RequestDevice,
  RequestTypes,
  DateTime,
} from '@gbaranski/types';
import { logIntervalStop, logSocketError } from '@/cli';

export default abstract class Device<DeviceData extends AnyDeviceData> {
  private _status = false;

  private _failedRequests = 0;

  abstract dataInterval: NodeJS.Timeout;

  constructor(
    private ws: WebSocket,
    private _deviceData: DeviceData,
    public readonly deviceName: string,
    public readonly deviceUid: string,
  ) {
    this._status = true;
  }

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

  protected async interval(): Promise<void> {
    const res = this.requestDevice(RequestTypes.GET_DATA);
    if (!(await res)) {
      console.log('Failed request');
      this._failedRequests += 1;
    }
    console.log(this._deviceData);
  }

  private stopDataInterval(): void {
    clearInterval(this.dataInterval);
  }

  public terminateConnection(reason: string): void {
    this.ws.terminate();
    this.stopDataInterval();
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

  get failedRequests(): number {
    return this._failedRequests;
  }
}
