import express from 'express';
import jwt from 'jsonwebtoken';
import { v4 as uuidv4 } from 'uuid';
import { validateDevice } from '@/services/firebase';

const router = express.Router();

router.post('/login', (req, res): void => {
  res.sendStatus(200);
});

router.get(
  '/getDeviceToken',
  async (req, res): Promise<void> => {
    const deviceType = req.get('deviceType');
    const secret = req.get('secret');
    const uid = req.get('uid');
    if (!deviceType || !secret || !uid) {
      res.sendStatus(400);
      return;
    }
    try {
      const isValid = await validateDevice(deviceType, uid, secret);
      if (!isValid) {
        throw new Error('Unauthorized');
      }
    } catch (e) {
      res.sendStatus(401);
      return;
    }
    const token = jwt.sign({ uid, deviceType }, process.env.JWT_KEY as string, {
      expiresIn: '5m',
    });
    res.type('html');
    res.send(token);
  },
);

export default router;
