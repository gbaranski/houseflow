import { decodeToken, validateDevice } from '@/services/firebase';
import express from 'express';
import { UserRequest } from './types';

const router = express.Router();

router.post('/user', async (req, res) => {
  const userRequest: UserRequest = req.body;
  try {
    if (userRequest.clientid.startsWith('device_')) {
      await validateDevice({
        uid: userRequest.username,
        secret: userRequest.password,
      });
    } else if (
      userRequest.clientid.startsWith('mobile_') ||
      userRequest.clientid.startsWith('web_')
    ) {
      await decodeToken(userRequest.password);
    } else {
      throw new Error('unrecognized client');
    }
    console.log(`Authorized ${userRequest.username}`);
    res.sendStatus(200);
  } catch (e) {
    console.log(
      `Failed authorization with ${userRequest.username} e ${e.message}`,
    );
    res.sendStatus(400);
  }
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
