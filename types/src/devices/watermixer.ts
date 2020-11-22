import { Client } from '../client';
import { Device } from '../device';

export namespace Watermixer {
  export const mixWaterRequestByUID = ({
    uid,
  }: {
    uid: string;
  }): Client.DeviceRequest['device'] => {
    return {
      uid,
      action: {
        name: Device.ActionName.MixWater,
        id: 0,
      },
    };
  };
}
