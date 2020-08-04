import express from 'express';
import jwt from 'jsonwebtoken';
import { authenticateDevice } from '@/auth';
import { getWssClients } from '@/services/websocket';

const router = express.Router();

router.post('/login', (req, res): void => {
  res.sendStatus(200);
});

router.get('/getToken', (req, res): void => {
  const device = req.get('device');
  const reqToken = req.get('token');
  if (!device || !reqToken) {
    res.sendStatus(400);
    return;
  }
  authenticateDevice(device, reqToken);
  const token = jwt.sign({ device }, process.env.JWT_KEY as string, {
    expiresIn: '5m',
  });
  res.type('html');
  res.send(token);
});

router.get('/getClients', (req, res): void => {
  res.send(JSON.stringify(getWssClients));
});

export default router;
