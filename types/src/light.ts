import { Client } from '.';

export namespace Light {
  export interface Data {
    currentState: boolean;
  }

  export const SAMPLE: Data = {
    currentState: false,
  };

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
