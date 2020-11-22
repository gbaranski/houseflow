import { Client } from './client';

export namespace Relay {
  export const createRelayRequestByUID = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequestByUID['device'] => {
    return {
      uid,
      action: 1,
    };
  };
}
