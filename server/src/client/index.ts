import WebSocket from 'ws';
import { validateSocketMessage } from '@/helpers';
import { logSocketError } from '@/cli';

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

  private _status = false;

  constructor(
    private readonly ws: WebSocket,
    public readonly clientUid: string,
  ) {
    this._status = true;
  }

  handleMessage(message: WebSocket.Data): void {
    validateSocketMessage(message);
    console.log(message);
  }

  public terminateConnection(reason: string): void {
    this.ws.terminate();
    logSocketError('Unknown', this.clientUid, reason, 'client');
  }

  get status(): boolean {
    return this._status;
  }
  set status(status: boolean) {
    this._status = status;
  }
}
