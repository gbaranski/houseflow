import { WatermixerData } from '@gbaranski/types';
import { devices } from '../globals';

export function watermixerInterval(): void {
  if (!devices.watermixer.ws) {
    return;
  }
  if (!devices.watermixer.status) {
    return;
  }
  devices.watermixer.ws.send('GET_DATA');
  devices.watermixer.ws.addEventListener(
    'message',
    (message: { data: string; type: string; target: WebSocket }) => {
      devices.watermixer.data = JSON.parse(message.data) as WatermixerData;
    },
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    { once: true },
  );
}
