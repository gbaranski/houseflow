import { validateDevice } from '@/services/firebase';
import { log } from '@/services/logging';
import chalk from 'chalk';
import express from 'express';
import Joi from 'joi';
import { AclRequest, UserRequest } from './types';

const deviceApiUsername = process.env.DEVICE_API_USERNAME;
const deviceApiPassword = process.env.DEVICE_API_PASSWORD;
if (!deviceApiUsername || !deviceApiPassword)
  throw new Error('Username or password is not defined in .env, read docs');

const router = express.Router();

const userRequestSchema = Joi.object({
  clientid: Joi.string().required(),
  ip: Joi.string().required(),
  username: Joi.string().length(36).required(),
  password: Joi.string().length(36).required(),
});

router.post('/user', async (req, res) => {
  const userRequest: UserRequest = req.body;

  try {
    const { error } = userRequestSchema.validate(req.body);
    if (error !== undefined) throw error;

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
    log(
      chalk.greenBright(
        `Authorized user ${userRequest.clientid} with username ${userRequest.username}`,
      ),
    );
    res.sendStatus(200);
  } catch (e) {
    log(
      chalk.redBright(
        `User auth failed: ${e.message} clientID: ${userRequest.clientid} username: ${userRequest.username}`,
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
    if (aclRequest.topic.includes('+'))
      throw new Error('not allowed wildcard(+) in topic');
    if (aclRequest.topic.includes('$'))
      throw new Error('not allowed SYS($) in topic');
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

    log(
      chalk.greenBright(
        `Authorized ACL ${aclRequest.clientid} with username ${
          aclRequest.username
        } on topic ${aclRequest.topic.slice(36)}`,
      ),
    );
    res.sendStatus(200);
  } catch (e) {
    log(
      chalk.redBright(
        `ACL Auth failed: ${e.message} client: ${aclRequest.username} topic: ${aclRequest.topic} ip: ${aclRequest.ip}`,
      ),
    );
    res.sendStatus(403);
    return;
  }
});

export default router;
