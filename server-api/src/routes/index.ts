import express from 'express';
import deviceRouter from './device/index';

const router = express.Router();

router.use('/device', deviceRouter)

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
