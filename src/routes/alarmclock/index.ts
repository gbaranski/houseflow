import express from 'express';
import { AlarmRequestType } from '@gbaranski/types';
import { fetchURL } from '../../helpers';
import { ALARMCLOCK_URL } from '../../config';
import { sendMessage } from '../../firebase';
import { Headers } from 'node-fetch';
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

router.get('/getTempArray', (req, res: express.Response): void => {
  res.json(JSON.stringify(devices.alarmclock.tempArray));
});

router.post(
  '/testSiren',
  async (req, res): Promise<void> => {
    if (!devices.alarmclock.ws) {
      res.sendStatus(503);
      return;
    }
    devices.alarmclock.ws.send('TEST_SIREN');
    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.TEST_ALARM}`,
    );
  },
);

router.post(
  '/setTime',
  async (req, res): Promise<void> => {
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
    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.SET_TIME}`,
    );
  },
);

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

    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.SWITCH_STATE}`,
    );
  },
);

export default router;
