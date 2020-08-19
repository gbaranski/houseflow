import http from 'http';

export interface VerifyInfo {
  origin: string;
  secure: boolean;
  req: http.IncomingMessage;
}

export interface VerifyCallback {
  (
    res: boolean,
    code?: number,
    message?: string,
    headers?: http.OutgoingHttpHeaders,
  ): void;
}

type SubChannelNames = 'request';
export interface SubChannel {
  name: SubChannelNames;
  handle: (message: string) => void;
}
