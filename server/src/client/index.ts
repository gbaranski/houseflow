import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import { validateSocketMessage } from '@/helpers';

export const currentClients: Array<WebSocketClient> = [];

export default class WebSocketClient {
  private static _currentClients: WebSocketClient[] = [];

  public static get currentDevices(): WebSocketClient[] {
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

  private _status = false;

  public readonly deviceUid: string = uuidv4();

  constructor(
    private readonly ws: WebSocket,
    public readonly deviceName: string,
  ) {
    this._status = true;
  }

  handleMessage(message: WebSocket.Data): void {
    validateSocketMessage(message);
  }

  get status(): boolean {
    return this._status;
  }
  set status(status: boolean) {
    this._status = status;
  }
}
