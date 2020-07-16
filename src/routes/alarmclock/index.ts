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
  res.json(JSON.stringify(getData()));
});

router.get('/getTempArray', (req, res: express.Response): void => {
  res.json(JSON.stringify(getTempArray()));
});

router.post(
  '/testSiren',
  async (req, res): Promise<void> => {
    res.sendStatus(await fetchURL(ALARMCLOCK_URL, AlarmRequestType.TEST_ALARM));
    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.TEST_ALARM}`,
    );
  },
);

router.post(
  '/setTime',
  async (req, res): Promise<void> => {
    const headers = new Headers();
    headers.append('time', req.header('time') || '');
    res
      .status(
        await fetchURL(ALARMCLOCK_URL, AlarmRequestType.SET_TIME, headers),
      )
      .end();

    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.SET_TIME}`,
    );
  },
);

router.post(
  '/switchState',
  async (req, res): Promise<void> => {
    const headers = new Headers();
    const state = req.header('state');
    if (!state) {
      res.status(400).end();
      return;
    }
    headers.append('state', state);
    res
      .status(
        await fetchURL(ALARMCLOCK_URL, AlarmRequestType.SWITCH_STATE, headers),
      )
      .end();
    sendMessage(
      req.header('username') || '',
      `alarmclock${AlarmRequestType.SWITCH_STATE}`,
    );
  },
);

export default router;
