import express from 'express';
import apiRouter from './api';
import alarmclockRouter from './alarmclock';
import watermixerRouter from './watermixer';

const router = express.Router();

router.use('/api', apiRouter);
router.use('/alarmclock', alarmclockRouter);
router.use('/watermixer', watermixerRouter);

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
