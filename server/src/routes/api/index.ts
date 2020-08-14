import express from 'express';
import jwt from 'jsonwebtoken';
import { validateDevice } from '@/services/firebase';

const router = express.Router();

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
      await validateDevice(deviceType, uid, secret);
    } catch (e) {
      console.log(e.message);
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
