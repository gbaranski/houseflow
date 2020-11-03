import mqttClient, { sendDeviceMessage } from '@/services/mqtt';
import { checkUserDeviceAccess, decodeToken } from '@/services/firebase';
import {
  logClientAuthError,
  logClientError,
  logServerError,
  logUnhandledError,
} from '@/services/logging';
import express from 'express';
import { validateType } from '@/utils';
import { Client, Exceptions } from '@houseflow/types';

const router = express();

router.use((req, res, next) => {
  if (!mqttClient.connected) throw new Error(Exceptions.MQTT_NOT_CONNECTED);
  next();
});

router.post('/', async (req, res) => {
  const { body } = req;
  try {
    const deviceRequest = JSON.parse(body);
    if (!validateType<Client.DeviceRequest>(deviceRequest))
      throw new Error(Exceptions.INVALID_DEVICE_REQUEST);
    const decodedUser = await decodeToken(deviceRequest.user.token);
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
    handleError(e, req, res);
  }
});

const handleError = (
  err: Error,
  req: express.Request,
  res: express.Response,
): void => {
  switch (err.message as Exceptions) {
    case Exceptions.INVALID_DEVICE_REQUEST:
      res.status(400).send(err.message);
      logServerError(err);
      break;
    case Exceptions.NO_DEVICE_ACCESS:
    case Exceptions.NO_USER_IN_DB:
      res.status(403).send(err.message);
      logClientAuthError(err);
      break;
    case Exceptions.MQTT_NOT_CONNECTED:
      res.status(503).send(err.message);
      logServerError(err);
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
