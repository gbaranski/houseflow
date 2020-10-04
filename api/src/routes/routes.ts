import express from 'express';
import device from './device';

const router = express.Router();

router.use('/device', device);

router.get('/', (req, res): void => {
  res.send('Hello from api server');
});

export default router;
