import { Device } from '@gbaranski/types';
import { getWebsocket } from '@/services/websocket';

export default () => {
  const mixWater = (uid: string) => {
    console.log('Mixing water');
    const req: Device.RequestDevice = {
      topic: {
        name: 'startmix',
        uid,
      }
    };
    getWebsocket()?.send(JSON.stringify(req));
  };

  return {
    mixWater,
  };
};
