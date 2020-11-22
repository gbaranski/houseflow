import { Client } from '.';

export namespace Light {
  export const createLightRequestByUID = ({
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
