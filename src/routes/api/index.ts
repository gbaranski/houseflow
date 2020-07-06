import express from 'express';
import { isAuthenticated } from '../../auth';
import { getHistory, getDeviceStatus } from '../globals';

const router = express.Router();

router.post('/login', (req, res): void => {
  const username = req.header('username');
  const password = req.header('password');
  if (!username || !password) {
    res.sendStatus(401);
    return;
  }

  if (isAuthenticated(username, password)) {
    res.sendStatus(200);
  } else {
    res.sendStatus(401);
  }
});

router.get('/getHistory', (req, res): void => {
  if (
    !isAuthenticated(req.header('username') || '', req.header('password') || '')
  ) {
    res.send(401).end();
  }
  res.json(getHistory());
});

router.get('/getDeviceStatus', (req, res): void => {
  if (
    !isAuthenticated(req.header('username') || '', req.header('password') || '')
  ) {
    res.send(401).end();
  }
  res.json(JSON.stringify(getDeviceStatus()));
});

export default router;
