import { Client } from '../client';
import { Device } from '../device';

export namespace Gate {
  export const openGateRequestByUID = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequest['device'] => {
    return {
      uid,
      action: {
        name: Device.ActionName.OpenGate,
        id: 0,
      },
    };
  };
}
