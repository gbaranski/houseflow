import express from 'express';
import deviceRequest from './request';

const router = express.Router();

router.get('/', (req, res): void => {
  res.send('Hello from Device API server');
});

router.use('/request', deviceRequest);

export default router;
