import { Device, AnyDeviceData } from '@gbaranski/types';
import DeviceModel from './models/deviceSchema';

export async function getAllActiveDevices(): Promise<Device.ActiveDevice[]> {
  const allDevicesDocs = await DeviceModel.find({});
  return Promise.all(allDevicesDocs.map((deviceDoc) => deviceDoc.toJSON()));
}

export async function initDeviceInDb(device: Device.ActiveDevice) {
  const newDeviceDb = new DeviceModel(device);
  await newDeviceDb.save();
}

export async function removeDeviceFromDb(device: Device.ActiveDevice) {}
