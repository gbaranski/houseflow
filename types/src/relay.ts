import { Client } from './client';

export namespace Relay {
  export const createRelayRequest = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequest['device'] => {
    return {
      uid,
      action: 1,
    };
  };
}
