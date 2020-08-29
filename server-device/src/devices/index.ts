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

  public status = false;

  public abstract handleMessage(message: WebSocket.Data): void;

  constructor(
    protected ws: WebSocket,
    public readonly firebaseDevice: DeviceType.FirebaseDevice,
    protected activeDevice: DeviceType.ActiveDevice,
  ) {
    this.initHandlers();
    Device.currentDeviceObjects.push(this);
    this.status = true;
  }

  private initHandlers() {
    const terminateConnection = (reason: string) => {
      this.terminateConnection(reason);
      clearInterval(pingInterval);
    };

    const pingInterval = setInterval(() => {
      if (!this.status) {
        return terminateConnection('Ping not received');
      }
      this.status = false;
      this.ws.ping();
    }, 2000);

    this.ws.on('message', (message) => this.handleMessage(message));
    this.ws.on('pong', () => {
      this.status = true;
    });
    this.ws.on('ping', () => {
      this.ws.pong();
    });
    this.ws.on('error', (err) => {
      console.log(err.message);
      terminateConnection(`Connection error UID: ${this.firebaseDevice.uid}`);
    });
    this.ws.on('close', (code, reason) => {
      terminateConnection(`Connection closed UID: ${this.firebaseDevice.uid} CODE: ${code} REASON: ${reason}`);
    });

  }

  public requestDevice(request: Client.Request): boolean {
    if (!this.ws) {
      throw new Error('Websocket is not defined');
    }
    if (!this.ws.OPEN) {
      throw new Error('Websocket is not at OPEN state');
    }
    if (!this.status) {
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
    this.status = false;
    publishDeviceDisconnect(this.activeDevice);
  }

}
