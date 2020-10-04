import express from 'express';
import mqttAuth from './mqtt';

const router = express.Router();

router.use('/mqtt', mqttAuth);

router.get('/', (req, res): void => {
  res.send('Hello from auth server');
});

export default router;
