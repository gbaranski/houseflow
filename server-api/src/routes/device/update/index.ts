import express, { Request, Response, NextFunction } from 'express';
import {
  validateDevice,
  DeviceCredentials,
  findDeviceInDatabase,
} from '@/services/firebase';
import { findBinaryFile, sendBinaryFile } from './misc';

const router = express.Router();

router.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.log(`${err.message} IP: ${req.headers.host}`);
  res.sendStatus(401);
});

router.use('/esp8266', (req, res, next) => {
  if (
    req.get('x-ESP8266-sdk-version') &&
    req.get('x-ESP8266-version') &&
    req.get('x-ESP8266-sketch-md5')
  ) {
    next();
  } else {
    throw new Error('Device headers are invalid');
  }
});

router.use(
  '/esp8266',
  async (req, res): Promise<any> => {
    const sketchMd5 = req.get('x-ESP8266-sketch-md5');
    const versionHeader = req.get('x-ESP8266-version');
    if (!versionHeader || !sketchMd5)
      throw new Error('Version and MD5 is not defined');

    const deviceCredentials: DeviceCredentials = JSON.parse(versionHeader);
    if (!deviceCredentials.secret || !deviceCredentials.uid)
      throw new Error('Device credentials are invalid');

    try {
      await validateDevice(deviceCredentials);
      const firebaseDevice = await findDeviceInDatabase(deviceCredentials.uid);
      const file = await findBinaryFile(firebaseDevice.type);

      if (file.md5 === sketchMd5) {
        console.log('MD5 is equal, skipping update');
        return res.sendStatus(304);
      } else {
        return sendBinaryFile(res, file);
      }
    } catch (e) {
      console.log(e.message);
      res.sendStatus(500);
    }
  },
);

export default router;
