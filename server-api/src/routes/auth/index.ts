import { validateDevice } from '@/services/firebase';
import express from 'express';

const router = express.Router();

router.post('/user', async (req, res) => {
  console.log('USER: ', req.body);
  res.sendStatus(200);
});

router.post('/acl', (req, res) => {
  console.log('ACL: ', req.body);
  res.sendStatus(200);
});

router.post('/superuser', (req, res) => {
  console.log('SuperUser: ', req.body);
  res.sendStatus(200);
  // const userData = getRequestData(req);
  // try {
  //   jwt.verify(userData.password, JWT_KEY);
  //   res.sendStatus(200);
  // } catch (e) {
  //   res.sendStatus(403);
  // }
});

export default router;
