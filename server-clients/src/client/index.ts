import WebSocket from 'ws';
import { DocumentReference } from '@/services/firebase';
import {
  Client,
  AnyDeviceData,
  CurrentConnections,
  Device,
} from '@gbaranski/types';
import { activeDevices } from '@/services/redis';

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

  private fullAcccessDevices: Device.FirebaseDevice[] = [];

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
      .catch((e) => console.error(e));
  }

  async setAccessDevices(): Promise<void> {
    this.fullAcccessDevices = await Promise.all(
      this.firebaseUser.devices.full_access.map(
        async (doc: DocumentReference): Promise<Device.FirebaseDevice> => {
          const deviceSnapshot = await doc.get();
          const deviceData = deviceSnapshot.data() as Partial<
            Device.FirebaseDevice
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
    this.websocket.on('message', (message) => this.handleMessage(message));
    this.websocket.on('error', (err) => {
      console.log(
        `Websocket error ${err.message} UID: ${this.firebaseUser.uid}`,
      );
    });
    this.websocket.on('close', (code, reason) => {
      console.log(`Websocket closed ${reason} UID: ${this.firebaseUser.uid}`);
      this.terminateConnection('Connection closed');
    });
  }

  private async getCurrentConnectionWithAccess(): Promise<
    Device.ActiveDevice[]
  > {
    return Promise.all(
      activeDevices.filter((device) =>
        this.fullAcccessDevices.some(
          (firebaseDevice) => firebaseDevice.uid === device.uid,
        ),
      ),
    );
  }

  private async getAllWebsocketConnections(): Promise<CurrentConnections> {
    const activeClients: Client.ActiveUser[] = WebSocketClient._currentClients.map(
      (client) => client.activeUser,
    );

    return {
      clients: activeClients,
      devices: activeDevices,
    };
  }

  private async interval(): Promise<void> {
    const clientResponse: Client.Response = {
      requestType: 'DATA',
      data: await this.getCurrentConnectionWithAccess(),
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
        if (this.firebaseUser.role !== 'admin') {
          console.log('No permissions');
          return;
        }
        const res: Client.Response = {
          requestType: 'CONNECTIONS',
          data: await this.getAllWebsocketConnections(),
        };
        this.websocket.send(JSON.stringify(res));
        return;
      }

      const deviceObject = (await this.getCurrentConnectionWithAccess()).find(
        (_deviceObject) => _deviceObject.uid === parsedMsg.deviceUid,
      );
      if (!deviceObject) throw new Error('Could not find device');
      // addRequest(parsedMsg);
    } catch (e) {
      console.error(e.message);
    }
  }

  public terminateConnection(reason: string): void {
    this.websocket.terminate();
    console.log(`Connection error ${reason} UID: ${this.firebaseUser.uid}`);
    WebSocketClient.removeClient(this);
  }
}
