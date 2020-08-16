import { Device, AnyDeviceData } from '@gbaranski/types';
import { DeviceModel } from './models';

export async function getAllActiveDevices(): Promise<Device.ActiveDevice[]> {
  const allDevicesDocs = await DeviceModel.find({});
  return Promise.all(allDevicesDocs.map((deviceDoc) => deviceDoc.toJSON()));
}
