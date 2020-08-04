import express from 'express';
import apiRouter from './api';

const router = express.Router();

router.use('/api', apiRouter);

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
