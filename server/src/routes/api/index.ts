import express from 'express';
import jwt from 'jsonwebtoken';
import { v4 as uuidv4 } from 'uuid';
import { authenticateDevice } from '@/auth';

const router = express.Router();

router.post('/login', (req, res): void => {
  res.sendStatus(200);
});

router.get('/getDeviceToken', (req, res): void => {
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

router.get('/getClientToken', (req, res): void => {
  const clientUid = uuidv4();
  const token = jwt.sign({ clientUid }, process.env.JWT_KEY as string, {
    expiresIn: '5m',
  });
  res.type('html');
  res.send(token);
});

export default router;
