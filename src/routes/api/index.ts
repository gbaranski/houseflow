import express from 'express';
import { getDeviceStatus } from '../globals';

const router = express.Router();

router.post('/login', (req, res): void => {
  res.sendStatus(200);
});

router.get('/getDeviceStatus', (req, res): void => {
  res.json(JSON.stringify(getDeviceStatus()));
});

export default router;
