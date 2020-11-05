import mqttClient, { sendDeviceMessage } from '@/services/mqtt';
import { checkUserDeviceAccess, decodeToken } from '@/services/firebase';
import {
  logClientAuthError,
  logClientError,
  logServerError,
  logUnhandledError,
} from '@/services/logging';
import express from 'express';
import Joi from 'joi';
import { Client, Exceptions } from '@houseflow/types';

const deviceRequestSchema = Joi.object({
  user: Joi.object({
    token: Joi.string().required(),
  }).required(),
  device: Joi.object({
    uid: Joi.string().length(36).required(),
    gpio: Joi.number().integer().min(0).required(),
    action: Joi.string().equal('trigger', 'toggle').required(),
    data: Joi.string(),
  }).required(),
});

const router = express();

router.use((req, res, next) => {
  if (!mqttClient.connected) throw new Error(Exceptions.MQTT_NOT_CONNECTED);
  next();
});

router.post('/', async (req, res) => {
  const { body } = req;
  let userUid = '';
  try {
    try {
      const { error } = deviceRequestSchema.validate(body);
      if (error !== undefined) throw new Error(Exceptions.INVALID_ARGUMENTS);
    } catch (e) {
      throw new Error(Exceptions.INVALID_ARGUMENTS);
    }
    const deviceRequest = body as Client.DeviceRequest;

    const decodedUser = await decodeToken(deviceRequest.user.token);
    userUid = decodedUser.uid;

    checkUserDeviceAccess({
      userUid: decodedUser.uid,
      deviceUid: deviceRequest.device.uid,
    });
    const result = await sendDeviceMessage(deviceRequest.device);
    if (result === Exceptions.SUCCESS) {
      res.status(200).send(Exceptions.SUCCESS);
    } else if (result === Exceptions.DEVICE_TIMED_OUT) {
      res.status(504).send(Exceptions.DEVICE_TIMED_OUT);
    }
  } catch (e) {
    handleError(e, req, res, userUid);
  }
});

const handleError = (
  err: Error,
  req: express.Request,
  res: express.Response,
  uid: string,
): void => {
  switch (err.message as Exceptions) {
    case Exceptions.INVALID_DEVICE_REQUEST:
      res.status(400).send(err.message);
      logServerError(err);
      break;
    case Exceptions.NO_DEVICE_ACCESS:
    case Exceptions.NO_USER_IN_DB:
      res.status(403).send(err.message);
      logClientAuthError(err, uid);
      break;
    case Exceptions.MQTT_NOT_CONNECTED:
      res.status(503).send(err.message);
      logServerError(err, uid);
      break;
    case Exceptions.INVALID_ARGUMENTS:
      res.status(400).send(err.message);
      logClientError(err);
      break;
    default:
      res.status(400).send(`ERROR: ${err.message}`);
      logUnhandledError(err);
      break;
  }
};

export default router;
