import { addDevice } from '@/database/mongo';
import express from 'express';
import fulfillment from './fulfillment';

const router = express.Router();

router.get('/', (_, res) => {
  res.send('Hello from Action server');
});

router.post('/fulfillment', fulfillment);
router.get('/requestSync', async (req, res) => {
  const userID = req.query['user_id'];
  if (!userID) {
    res.status(400).send('user_id not defined');
    return;
  }
  if (typeof userID !== 'string') {
    res.status(400).send('user_id is invalid');
    return;
  }
  try {
    const result = await fulfillment.requestSync(userID);

    res.status(200).send(`OK: ${result}`);
  } catch (e) {
    res.status(500).send(e);
  }
});

router.get('/add1', async (_, res) => {
  addDevice({
    type: 'action.devices.types.LIGHT',
    traits: ['action.devices.traits.OnOff'],
    name: {
      defaultNames: ['Night lamp', 'Bedside lamp'],
      name: 'Night lamp',
      nicknames: ['Night lamp'],
    },
    deviceInfo: {
      manufacturer: 'gbaranski`s garage',
      model: 'nightlamp',
      hwVersion: '1.0',
      swVersion: '1.0',
    },
    willReportState: true,
    roomHint: 'Bedroom',
    state: {
      online: true,
      on: true,
    },
  });
  res.status(200).send();
});

export default router;
