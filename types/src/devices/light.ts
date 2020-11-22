import { Client } from '../client';
import { Device } from '../device';

export namespace Light {
  export const switchLightRequestByUID = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequest['device'] => {
    return {
      uid,
      action: {
        name: Device.ActionName.SwitchLights,
        id: 0,
      },
    };
  };
}
