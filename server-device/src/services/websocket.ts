import { VerifyInfo, VerifyCallback } from '@/types';
import { Watermixer, Alarmclock } from '@gbaranski/types';
import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { getIpStr } from '@/services/misc';
import WatermixerDevice from '@/devices/watermixer';
import AlarmclockDevice from '@/devices/alarmclock';
import { convertToFirebaseDevice, validateDevice } from '@/services/firebase';

export const verifyDevice = async (
  info: VerifyInfo,
  callback: VerifyCallback,
): Promise<void> => {
  const uid = info.req.headers.uid;
  const secret = info.req.headers.secret;

  try {
    if (!uid || !secret) throw new Error("UID or secret not defined");
    if (uid instanceof Array || secret instanceof Array) throw new Error("UID cannot be instance of an array");

    console.log(
      `UID ${uid} connection attempt`,
    );
    await validateDevice(uid, secret);
    callback(true);
  } catch (e) {
    console.log(`${uid} failed due to ${e.message}`);
    callback(false, 400, e.message);
  }
};

export const onConnection = async (ws: WebSocket, req: IncomingMessage) => {
  const { uid } = req.headers;
  try {
    if (!uid || uid instanceof Array) throw new Error("UID is undefined/invalid");

    const firebaseDevice = await convertToFirebaseDevice(uid);
    const ip = getIpStr(req);

    switch (firebaseDevice.type) {
      case 'WATERMIXER':
        new WatermixerDevice(ws, firebaseDevice, {
          ...firebaseDevice,
          data: Watermixer.SAMPLE,
          ip,
        });
        break;

      case 'ALARMCLOCK':
        new AlarmclockDevice(ws, firebaseDevice, {
          ...firebaseDevice,
          data: Alarmclock.SAMPLE,
          ip,
        });
        break;

      default:
        throw new Error("failed recognizing")
        break;
    }

    console.log(`UID: ${uid} IP:${getIpStr(req)} connected`);
  } catch (e) {
    ws.terminate();
    console.error(`UID: ${uid} failed due to ${e.message}`);
    return;
  }
};
