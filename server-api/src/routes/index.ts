import express from 'express';
import deviceRouter from './device';
import authRouter from './auth';

const router = express.Router();

router.use('/device', deviceRouter);
router.use('/auth', authRouter);

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
