import express from 'express';
import { RequestHistory, AlarmRequestType } from '@gbaranski/types';
import { createHistory, setProcessing, getProcessing } from '../globals';
import { getIpStr, fetchURL } from '../../helpers';
import { getData, getTempArray } from './interval';
import { ALARMCLOCK_URL } from '../../config';
import { sendMessage } from '../../firebase';

const setProcessingAlarmclock = (state: boolean) => {
  setProcessing({
    ...getProcessing(),
    alarmclock: state,
  });
};

const router = express.Router();

router.get('/getData', (req, res) => {
  res.json(JSON.stringify(getData()));
});

router.get('/getTempArray', (req, res) => {
  res.json(JSON.stringify(getTempArray()));
});

router.post('/testSiren', async (req, res) => {
  setProcessingAlarmclock(true);
  res.status(await fetchURL(ALARMCLOCK_URL, AlarmRequestType.TEST_ALARM, new Headers())).end();
  setProcessingAlarmclock(true);
  sendMessage(req.header('username') || '', `alarmclock${AlarmRequestType.TEST_ALARM}`);

  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.TEST_ALARM,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

router.post('/setTime', async (req, res) => {
  const headers = new Headers();
  headers.append('time', req.header('time') || '');
  res.status(await fetchURL(ALARMCLOCK_URL, AlarmRequestType.SET_TIME, headers)).end();

  sendMessage(req.header('username') || '', `alarmclock${AlarmRequestType.SET_TIME}`);

  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SET_TIME,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

router.post('/switchState', async (req, res) => {
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SWITCH_STATE,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);

  const headers = new Headers();
  const state = req.header('state');
  if (!state) {
    res.status(400).end();
    return;
  }
  headers.append('state', state);
  res.status(await fetchURL(ALARMCLOCK_URL, AlarmRequestType.SWITCH_STATE, headers)).end();
  sendMessage(req.header('username') || '', `alarmclock${AlarmRequestType.SWITCH_STATE}`);
});
