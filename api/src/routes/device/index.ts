import express from 'express';

const router = express.Router();

router.get('/connected', (req, res) => {
  console.log('Connected: ', req.body);
  res.sendStatus(200);
});

router.get('/disconnected', (req, res) => {
  console.log('Disconnected: ', req.body);
  res.sendStatus(200);
});

export default router;
