import { Client } from '@gbaranski/types';
import { getWebsocket } from '@/services/websocket';

export default () => {
  const mixWater = (uid: string) => {
    console.log('Mixing water');
    const req: Client.Request = {
      deviceUid: uid,
      requestType: 'START_MIXING',
    };
    getWebsocket()?.send(JSON.stringify(req));
  };

  return {
    mixWater,
  };
};
