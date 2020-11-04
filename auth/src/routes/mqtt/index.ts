import { validateDevice } from '@/services/firebase';
import chalk from 'chalk';
import express from 'express';
import { AclRequest, UserRequest } from './types';

const deviceApiUsername = process.env.DEVICE_API_USERNAME;
const deviceApiPassword = process.env.DEVICE_API_PASSWORD;
if (!deviceApiUsername || !deviceApiPassword)
  throw new Error('Username or password is not defined in .env, read docs');

const router = express.Router();

router.post('/user', async (req, res) => {
  const userRequest: UserRequest = req.body;
  try {
    if (userRequest.username === deviceApiUsername) {
      if (userRequest.password === deviceApiPassword) {
        res.sendStatus(200);
        return;
      } else {
        throw new Error(
          'URGENT: attempt to log in with restricted username or invalid password',
        );
      }
    }
    if (!userRequest.clientid.startsWith('device_'))
      throw new Error('expected device_... clientid');
    await validateDevice({
      uid: userRequest.username,
      secret: userRequest.password,
    });
    console.log(
      chalk.greenBright(
        `Authorized user ${userRequest.clientid} with username ${userRequest.username}`,
      ),
    );
    res.sendStatus(200);
  } catch (e) {
    console.log(
      chalk.redBright(
        `User auth failed due ${e.message} clientID: ${userRequest.clientid} username: ${userRequest.username}`,
      ),
    );
    res.sendStatus(400);
  }
});

router.post('/acl', (req, res) => {
  const aclRequest: AclRequest = req.body;
  if (aclRequest.username === deviceApiUsername) {
    res.sendStatus(200);
    return;
  }

  try {
    const expectedClientIdPrefix = 'device_';
    const expectedTopicPrefix = `${aclRequest.username}/`;
    const expectedTopicSuffix =
      aclRequest.access === '1' ? '/request' : '/response';

    if (aclRequest.topic.includes('#'))
      throw new Error('not allowed wildcard(#) in topic');
    if (!aclRequest.clientid.startsWith(expectedClientIdPrefix))
      throw new Error(
        `not allowed clientid, expected prefix ${expectedClientIdPrefix}, recived ${aclRequest.clientid}`,
      );
    if (!aclRequest.topic.startsWith(expectedTopicPrefix))
      throw new Error(
        `not allowed topic, expected prefix ${expectedTopicPrefix}, received ${aclRequest.topic}`,
      );

    if (!aclRequest.topic.endsWith(expectedTopicSuffix))
      throw new Error(
        `not allowed topic, expected suffix ${expectedTopicSuffix}, received ${aclRequest.topic}`,
      );

    console.log(
      chalk.greenBright(
        `Authorized ACL ${aclRequest.clientid} with username ${
          aclRequest.username
        } on topic ${aclRequest.topic.slice(36)}`,
      ),
    );
    res.sendStatus(200);
  } catch (e) {
    console.log(
      chalk.redBright(
        `ACL Auth failed due ${e.message} client: ${aclRequest.username} topic: ${aclRequest.topic} ip: ${aclRequest.ip}`,
      ),
    );
    res.sendStatus(403);
    return;
  }
});

export default router;
