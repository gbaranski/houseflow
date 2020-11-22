import { Client } from '../client';
import { Device } from '../device';

export namespace Garage {
  export const openGarageRequestByUID = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequest['device'] => {
    return {
      uid,
      action: {
        name: Device.ActionName.OpenGarage,
        id: 0,
      },
    };
  };
}
