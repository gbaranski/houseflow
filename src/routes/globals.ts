/* eslint-disable no-console */
import { DeviceStatus, RequestHistory, Devices } from '@gbaranski/types';
import fs from 'fs-extra';

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

const historyFile = '@/log/req_history.json';

async function createFile() {
  await fs.createFile(historyFile);
  await fs.writeJson(historyFile, []);
  return 0;
}

export async function createHistory(data: RequestHistory) {
  try {
    if (!(await fs.pathExists(historyFile))) {
      await createFile();
    }
    const fileData = await fs.readJson(historyFile);
    fileData.push(data);
    await fs.outputJson(historyFile, fileData);
  } catch (e) {
    console.log(e);
  }
}

export function getHistory() {
  return fs.readJsonSync(historyFile);
}

export function setProcessing(newIsProcessing: DeviceStatus) {
  isProcessing = newIsProcessing;
}

export function getProcessing(): DeviceStatus {
  return isProcessing;
}
