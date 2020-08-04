/* eslint-disable no-console */
import Watermixer from '@/devices/watermixer';
import Alarmclock from '@/devices/alarmclock';

export type AnyDeviceObject = Watermixer | Alarmclock;

export const currentDevices: Array<AnyDeviceObject> = [];
