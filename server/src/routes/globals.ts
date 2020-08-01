/* eslint-disable no-console */
import { devicesSample, Devices } from '@gbaranski/types';

export const devices: Devices = {
  ...devicesSample,
};

export const getDeviceStatus = (): {
  alarmclock: boolean;
  watermixer: boolean;
} => {
  return {
    alarmclock: devices.alarmclock.status,
    watermixer: devices.watermixer.status,
  };
};
