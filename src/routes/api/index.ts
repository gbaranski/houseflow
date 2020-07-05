import express from 'express';

const router = express.Router();

router.post('/login', (req, res) => {
  const username = req.header('username');
  const password = req.header('password');
  if (username && password) {
    if (isAuthenticated(username, password)) {
      res.send(200).end();
    } else {
      res.send(401).end();
    }
  } else {
    res.send(401).end();
  }
});

router.get('/getHistory', (req, res) => {
  if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
    res.send(401).end();
  }
  res.json(getHistory());
});

router.get('/getDeviceStatus', (req, res) => {
  if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
    res.send(401).end();
  }
  res.json(JSON.stringify(deviceStatus));
});

export default router;
