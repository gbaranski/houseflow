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

type ChannelNames = 'device_data' | 'device_disconnect';
export interface Channel {
  name: ChannelNames;
  handle: (message: string) => void;
}
