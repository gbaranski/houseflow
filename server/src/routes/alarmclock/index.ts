import express from 'express';
import { devices } from '../globals';

export function setAlarmclockState(state: boolean): void {
  devices.alarmclock.status = state;
}
export function getAlarmclockState(): boolean {
  return devices.alarmclock.status;
}

const router = express.Router();

router.get('/getData', (req, res: express.Response): void => {
  res.json(JSON.stringify(devices.alarmclock.data));
});

router.post(
  '/testSiren',
  async (req, res): Promise<void> => {
    if (!devices.alarmclock.ws) {
      res.sendStatus(503);
      return;
    }
    devices.alarmclock.ws.send('TEST_SIREN');
    res.sendStatus(201);
  },
);

router.post('/setTime', (req, res): void => {
  if (!devices.alarmclock.ws) {
    res.sendStatus(503);
    return;
  }
  const time = req.get('time');
  if (!time) {
    res.sendStatus(400);
    return;
  }
  devices.alarmclock.ws.send(`TIME=${time}`);
  res.sendStatus(201);
});

router.post(
  '/switchState',
  async (req, res): Promise<void> => {
    if (!devices.alarmclock.ws) {
      res.sendStatus(503);
      return;
    }
    const state = req.header('state');
    if (!state) {
      res.sendStatus(400);
      return;
    }
    devices.alarmclock.ws.send(`STATE=${state}`);
    res.sendStatus(201);
  },
);

export default router;
