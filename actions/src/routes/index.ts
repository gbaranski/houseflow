import express from 'express';
import fulfillment from './fulfillment';

const router = express.Router();

router.get('/', (req, res) => {
  res.send('Hello from Action server');
});

router.post('/fulfillment', fulfillment);

export default router;
