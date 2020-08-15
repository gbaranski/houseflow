import WebSocket from 'ws';
import { logSocketError, logError } from '@/cli';
import { DocumentReference } from '@/services/firebase';
import {
  Device as DeviceType,
  Client,
  AnyDeviceData,
  CurrentConnections,
} from '@gbaranski/types';
import Device, { AnyDeviceObject } from '@/devices';
import { GET_CONNECTIONS_MIN_PERM } from '@/config/permissions';

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

  private fullAcccessDevices: DeviceType.FirebaseDevice[] = [];

  constructor(
    private websocket: WebSocket,
    public readonly firebaseUser: Client.FirebaseUser,
    public readonly activeUser: Client.ActiveUser,
  ) {
    this.setWebsocketHandling();
    this.setAccessDevices()
      .then(() => {
        WebSocketClient.addNewClient(this);
        this.interval();
        setInterval(() => this.interval(), 1000);
      })
      .catch(e => console.error(e));
  }

  async setAccessDevices(): Promise<void> {
    this.fullAcccessDevices = await Promise.all(
      this.firebaseUser.devices.full_access.map(
        async (doc: DocumentReference): Promise<DeviceType.FirebaseDevice> => {
          const deviceSnapshot = await doc.get();
          const deviceData = deviceSnapshot.data() as Partial<
            DeviceType.FirebaseDevice
          >;

          if (!deviceData.type) throw new Error('Type does not exist!');

          return {
            type: deviceData.type,
            uid: deviceSnapshot.id,
          };
        },
      ),
    );
  }

  private setWebsocketHandling() {
    this.websocket.on('message', message => this.handleMessage(message));
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
      this.fullAcccessDevices.some(
        firebaseDevice => firebaseDevice.uid === device.firebaseDevice.uid,
      ),
    );
  }

  private getAllWebsocketConnections(): CurrentConnections {
    const activeClients: Client.ActiveUser[] = WebSocketClient._currentClients.map(
      client => client.activeUser,
    );

    const activeDevices: DeviceType.ActiveDevice<
      AnyDeviceData
    >[] = Device.currentDevices.map(
      (device): DeviceType.ActiveDevice<AnyDeviceData> => device.activeDevice,
    );
    return {
      clients: {
        online: activeClients,
        offline: [],
      },
      devices: {
        online: activeDevices,
        offline: [],
      },
    };
  }

  private async interval(): Promise<void> {
    const deviceData: DeviceType.ActiveDevice<
      AnyDeviceData
    >[] = this.getCurrentConnectionWithAccess().map(
      (deviceObject): DeviceType.ActiveDevice<AnyDeviceData> =>
        deviceObject.activeDevice,
    );
    const clientResponse: Client.Response = {
      requestType: 'DATA',
      data: deviceData,
    };
    this.websocket.send(JSON.stringify(clientResponse));
  }

  private static parseMessage(message: WebSocket.Data): Client.Request {
    const parsedMsg = (message as unknown) as Client.Request;
    if (!parsedMsg.deviceUid && parsedMsg.requestType !== 'CONNECTIONS')
      throw new Error('Uid is missing');
    if (!parsedMsg.requestType) throw new Error('Request type is missing');
    return parsedMsg;
  }

  private async handleMessage(message: WebSocket.Data): Promise<void> {
    try {
      if (
        message instanceof Buffer ||
        message instanceof Array ||
        message instanceof ArrayBuffer
      )
        throw new Error('Wrong message type');
      const parsedMsg = WebSocketClient.parseMessage(JSON.parse(message));

      if (parsedMsg.requestType === 'CONNECTIONS') {
        console.log('Someone requesting connections!');
        if (this.firebaseUser.permission < GET_CONNECTIONS_MIN_PERM) {
          console.log('No permissions');
          return;
        }
        const res: Client.Response = {
          requestType: 'CONNECTIONS',
          data: this.getAllWebsocketConnections(),
        };
        this.websocket.send(JSON.stringify(res));
        return;
      }

      const deviceObject = this.getCurrentConnectionWithAccess().find(
        _deviceObject =>
          _deviceObject.firebaseDevice.uid === parsedMsg.deviceUid,
      );
      if (!deviceObject) throw new Error('Could not find device');
      deviceObject.requestDevice(parsedMsg.requestType, parsedMsg.data);
    } catch (e) {
      console.error(e.message);
    }
  }

  public terminateConnection(reason: string): void {
    this.websocket.terminate();
    logSocketError('Unknown', this.firebaseUser.uid, reason, 'client');
    WebSocketClient.removeClient(this);
  }
}
