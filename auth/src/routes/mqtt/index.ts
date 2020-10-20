import {
  decodeToken,
  firebaseUsers,
  validateDevice,
} from '@/services/firebase';
import express from 'express';
import { AclRequest, UserRequest } from './types';

const UUID_LENGTH = 36;

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
    console.log(
      `Authorized user ${userRequest.clientid} with username ${userRequest.username}`,
    );
    res.sendStatus(200);
  } catch (e) {
    console.log(
      `Failed user authorization with ${userRequest.username} ${e.message}`,
    );
    res.sendStatus(400);
  }
});

router.post('/acl', (req, res) => {
  const aclRequest: AclRequest = req.body;
  try {
    if (aclRequest.clientid.startsWith('device_')) {
      if (!aclRequest.topic.startsWith(`${aclRequest.username}/`))
        throw new Error('not allowed topic');
    } else if (
      aclRequest.clientid.startsWith('web_') ||
      aclRequest.clientid.startsWith('mobile_')
    ) {
      const firebaseUser = firebaseUsers.find(
        (user) => user.uid === aclRequest.username,
      );
      if (!firebaseUser) throw new Error(' firebase user not found');
      const toSubscribeDeviceUid = aclRequest.topic.substring(0, UUID_LENGTH);
      if (
        !firebaseUser.devices.find(
          (device) => device.uid === toSubscribeDeviceUid,
        )
      )
        throw new Error('not allowed device topic');
    }
    console.log(
      `Successfully authenticated ${aclRequest.clientid} with username ${aclRequest.username}`,
    );
    res.sendStatus(200);
  } catch (e) {
    console.log(
      `ACL Auth failed due ${e.message} client: ${aclRequest.username} topic: ${aclRequest.topic} ip: ${aclRequest.ip}`,
    );
    res.sendStatus(400);
    return;
  }
});

export default router;
