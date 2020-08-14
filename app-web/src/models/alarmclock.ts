import { Client } from '@gbaranski/types';
import { getWebsocket } from '@/services/websocket';

export default () => {
  const testSiren = (uid: string) => {
    console.log('Mixing water');
    const req: Client.Request = {
      deviceUid: uid,
      requestType: 'TEST_SIREN',
    };
    getWebsocket()?.send(JSON.stringify(req));
  };

  return {
    testSiren,
  };
};
