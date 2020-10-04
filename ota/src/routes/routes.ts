import express from 'express';
import esp8266 from './esp8266';

const router = express.Router();

router.use('/esp8266', esp8266);

router.get('/', (req, res): void => {
  res.send('Hello from ota server');
});

export default router;
