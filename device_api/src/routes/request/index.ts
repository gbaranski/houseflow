import mqttClient, { sendDeviceMessage } from '@/services/mqtt';
import {
  Access,
  addRequestHistory,
  checkUserDeviceAccess,
  decodeToken,
  findDeviceByAction,
  getFirebaseUserByUid,
} from '@/services/firebase';
import {
  logClientAuthError,
  logClientError,
  logServerError,
  logUnhandledError,
} from '@/services/logging';
import express from 'express';
import Joi from 'joi';
import { Client, Device, Exceptions } from '@houseflow/types';

const deviceRequestSchema = Joi.object({
  user: Joi.object({
    token: Joi.string().required(),
    geoPoint: Joi.object({
      latitude: Joi.number().required(),
      longitude: Joi.number().required(),
    }).required(),
  }).required(),
  device: Joi.object({
    action: Joi.object({
      name: Joi.string()
        .valid(...Object.values(Device.ActionName))
        .required(),
      id: Joi.number().required(),
    }).required(),
    uid: Joi.string().length(36),
    data: Joi.string(),
  }).required(),
});

const router = express();

router.use((req, res, next) => {
  if (!mqttClient.connected) throw new Error(Exceptions.MQTT_NOT_CONNECTED);
  next();
});

const onUnknownUIDRequest = async (
  req: Client.DeviceRequest,
  userUID: string,
): Promise<string> => {
  const firebaseUser = getFirebaseUserByUid(userUID);
  if (!firebaseUser) throw new Error(Exceptions.NO_USER_IN_DB);
  const firebaseDevice = await findDeviceByAction(
    req.device.action,
    firebaseUser,
  );
  return firebaseDevice.uid;
};

const onKnownUIDRequest = (
  req: Client.DeviceRequest,
  userUID: string,
): string => {
  if (!req.device.uid) throw new Error('Expected device UID to be defined');
  checkUserDeviceAccess({
    userUid: userUID,
    deviceUid: req.device.uid,
    access: Access.EXECUTE,
  });
  return req.device.uid;
};

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

    const deviceUid = deviceRequest.device.uid
      ? onKnownUIDRequest(deviceRequest, userUid)
      : await onUnknownUIDRequest(deviceRequest, userUid);

    const result = await sendDeviceMessage(
      deviceUid,
      deviceRequest.device.action,
    );

    if (result === Exceptions.SUCCESS) {
      res.status(200).send(Exceptions.SUCCESS);
      await addRequestHistory({
        request: deviceRequest,
        ipAddress: (req.headers['x-forwarded-for'] ||
          req.connection.remoteAddress) as string,
        userUid: userUid,
        deviceUid: deviceUid,
      });
    } else if (result === Exceptions.DEVICE_TIMED_OUT) {
      res.status(504).send(Exceptions.DEVICE_TIMED_OUT);
      return;
    } else {
      res.status(500).send('Unknown error');
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
