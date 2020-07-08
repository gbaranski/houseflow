/* eslint-disable no-console */
import { DeviceStatus } from '@gbaranski/types';

const deviceStatusPattern = {
  alarmclock: false,
  watermixer: false,
  gate: false,
  garage: false,
};

let deviceStatus: DeviceStatus = {
  ...deviceStatusPattern,
};

let isProcessing: DeviceStatus = {
  ...deviceStatusPattern,
};

export function setDeviceStatus(newStatus: DeviceStatus): void {
  deviceStatus = newStatus;
}

export function getDeviceStatus(): DeviceStatus {
  return deviceStatus;
}

export function setProcessing(newIsProcessing: DeviceStatus): void {
  isProcessing = newIsProcessing;
}

export function getProcessing(): DeviceStatus {
  return isProcessing;
}
