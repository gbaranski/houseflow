import { JWT_KEY } from '@/index';
import { validateDevice } from '@/services/firebase';
import express from 'express';
import jwt from 'jsonwebtoken';

interface RequestData {
  clientid: string;
  username: string;
  password: string;
}

const getRequestData = (req: express.Request): RequestData => ({
  clientid: req.body.clientid,
  username: req.body.username,
  password: req.body.password,
});

const validateForDevice = async (requestData: RequestData) => {
  await validateDevice({
    uid: requestData.username,
    secret: requestData.password,
  });
};

const validateForServerDevice = (requestData: RequestData) => {
  jwt.verify(requestData.password, JWT_KEY);
};

const router = express.Router();
router.post('/user', async (req, res) => {
  const requestData = getRequestData(req);
  try {
    if (requestData.clientid.startsWith('server-device')) {
      validateForServerDevice(requestData);
    } else {
      await validateForDevice(requestData);
    }
    res.sendStatus(200);
  } catch (e) {
    console.log(`${requestData.clientid} failed due to ${e.message} `);
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
