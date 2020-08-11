import WebSocket from 'ws';
import { logSocketError, logError } from '@/cli';
import { convertToFirebaseUser, DocumentReference } from '@/services/firebase';
import {
  FirebaseDevice,
  DeviceType,
  DeviceStatus,
  CurrentDevice,
  ResponseClient,
  WatermixerData,
  AlarmclockData,
  ClientRequests,
  RequestClient,
} from '@gbaranski/types';
import Device, { AnyDeviceObject } from '@/devices';

export default class WebSocketClient {
  private static _currentClients: WebSocketClient[] = [];

  public static get currentClients(): WebSocketClient[] {
    return this._currentClients;
  }

  public static addNewClient(client: WebSocketClient): void {
    this._currentClients.push(client);
  }

  public static removeClient(client: WebSocketClient): void {
    this._currentClients = this._currentClients.filter(
      (_client: WebSocketClient) => _client !== client,
    );
  }
  private static isDeviceCurrentlyConnected(deviceUid: string): boolean {
    return Device.currentDevices.some(
      activeDevice => deviceUid === activeDevice.deviceUid,
    );
  }

  private status = false;

  public userPermission: number | undefined;

  private fullAccessCurrentDevices: CurrentDevice[] = [];

  constructor(private websocket: WebSocket, public readonly clientUid: string) {
    this.setWebsocketHandling();
    this.setAccessDevices()
      .then(() => {
        setInterval(() => this.interval(), 1000);
      })
      .catch(e => console.error(e));
  }

  async setAccessDevices(): Promise<void> {
    const firebaseUser = await convertToFirebaseUser(this.clientUid);
    this.userPermission = firebaseUser.permission;

    firebaseUser.devices.full_access.forEach(async (doc: DocumentReference) => {
      const deviceSnapshot = await doc.get();
      const deviceData = deviceSnapshot.data() as Partial<FirebaseDevice>;

      if (!deviceData.secret) throw Error('Secret does not exist!');
      if (!deviceData.type) throw new Error('Type does not exist!');

      const currentDevice: CurrentDevice = {
        type: DeviceType[deviceData.type],
        secret: deviceData.secret,
        uid: deviceSnapshot.id,
      };

      this.fullAccessCurrentDevices.push(currentDevice);
    });
  }

  private setWebsocketHandling() {
    this.websocket.on('message', this.handleMessage);
    this.websocket.on('error', err => {
      logError(err.message);
    });
    this.websocket.on('close', (code, reason) => {
      logError(`CODE: ${code} \nREASON:${reason}`);
      this.terminateConnection('Connection closed');
    });
  }

  private getCurrentConnectionWithAccess(): AnyDeviceObject[] {
    return Device.currentDevices.filter(device =>
      this.fullAccessCurrentDevices.some(
        firebaseDevice => firebaseDevice.uid === device.deviceUid,
      ),
    );
  }

  private async interval(): Promise<void> {
    this.getCurrentConnectionWithAccess().forEach(deviceObject => {
      if (deviceObject.deviceType === DeviceType.ALARMCLOCK) {
        // fix this kaszana
        const deviceData: ResponseClient<AlarmclockData> = {
          ok: true,
          deviceUid: deviceObject.deviceUid,
          deviceType: DeviceType.ALARMCLOCK,
          responseFor: ClientRequests.GET_DATA,
          data: deviceObject.deviceData as AlarmclockData,
        };
        this.websocket.send(JSON.stringify(deviceData));
      } else if (deviceObject.deviceType === DeviceType.WATERMIXER) {
        const deviceData: ResponseClient<WatermixerData> = {
          ok: true,
          deviceUid: deviceObject.deviceUid,
          deviceType: DeviceType.WATERMIXER,
          responseFor: ClientRequests.GET_DATA,
          data: deviceObject.deviceData as WatermixerData,
        };
        this.websocket.send(JSON.stringify(deviceData));
      }
    });
  }

  private async handleMessage(message: WebSocket.Data): Promise<void> {
    console.log({ message });
    const request = JSON.parse(message as string) as RequestClient;
    console.log(this.websocket);
    if (request.type === ClientRequests.GET_DEVICES_STATUS) {
      console.log(this.fullAccessCurrentDevices);
    }
  }

  public terminateConnection(reason: string): void {
    this.websocket.terminate();
    logSocketError('Unknown', this.clientUid, reason, 'client');
    WebSocketClient.removeClient(this);
  }

  private getDevicesStatus(): DeviceStatus[] {
    const deviceStatus: DeviceStatus[] = [];
    this.fullAccessCurrentDevices.forEach(currentDevice => {
      const _deviceStatus: DeviceStatus = {
        deviceUid: currentDevice.uid,
        status: WebSocketClient.isDeviceCurrentlyConnected(currentDevice.uid),
      };
      deviceStatus.push(_deviceStatus);
    });
    return deviceStatus;
  }
}
