import { Client } from '.';

export namespace Light {
  export const createLightRequest = ({
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
