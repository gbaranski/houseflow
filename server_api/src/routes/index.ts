import express from 'express';
import deviceRouter from './device';
import authRouter from './mqtt';

const router = express.Router();

router.use('/device', deviceRouter);
router.use('/mqtt', authRouter);

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
