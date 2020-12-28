import fs from 'fs';
import { smarthome } from 'actions-on-google';
import { extractJWTToken } from '@/utils';
import { verifyToken } from '@/services/token';
import { fetchTokenUUID } from '@/services/redis';

if (!process.env.HOMEGRAPH_API_KEY)
  throw new Error('HOMEGRAPH_API_KEY not set in .env');

const serviceAccountKey = JSON.parse(
  fs.readFileSync('/app/service-account.json', 'utf8'),
);

const app = smarthome({
  debug: true,
  key: process.env.HOMEGRAPH_API_KEY,
  jwt: serviceAccountKey,
});

app.onSync(async (body, headers) => {
  console.log('Received sync request');
  const token = extractJWTToken(headers);
  console.log({ token });
  const claims = await verifyToken(token);
  console.log({ claims });
  const userID = await fetchTokenUUID(claims.jti);
  console.log({ userID });

  return {
    requestId: body.requestId,
    payload: {
      agentUserId: userID,
      devices: [
        {
          id: 'washer',
          type: 'action.devices.types.WASHER',
          traits: [
            'action.devices.traits.OnOff',
            'action.devices.traits.StartStop',
            'action.devices.traits.RunCycle',
          ],
          name: {
            defaultNames: ['My Washer'],
            name: 'Washer',
            nicknames: ['Washer'],
          },
          deviceInfo: {
            manufacturer: 'Acme Co',
            model: 'acme-washer',
            hwVersion: '1.0',
            swVersion: '1.0.1',
          },
          willReportState: true,
          attributes: {
            pausable: true,
          },
        },
      ],
    },
  };
});

export default app;
