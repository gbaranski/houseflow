import express from 'express';
import { getDeviceStatus } from '../globals';
import jwt from 'jsonwebtoken';

const router = express.Router();

router.post('/login', (req, res): void => {
  res.sendStatus(200);
});

router.get('/getDeviceStatus', (req, res): void => {
  res.json(JSON.stringify(getDeviceStatus()));
});

router.get('/getToken', (req, res): void => {
  const token = jwt.sign({ foo: 'bar' }, process.env.JWT_KEY as string, {
    expiresIn: '1h',
  });
  res.send(token);
});

export default router;
