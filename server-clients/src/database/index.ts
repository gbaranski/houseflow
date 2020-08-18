import mongoose from 'mongoose';
import { Client, Device } from '@gbaranski/types';
import { RequestModel, DeviceModel } from './models';

const parseRequest = (request: Client.Request) => {
  return {
    ...request,
    data: request.data ? JSON.stringify(request.data) : undefined,
  };
};

const parseData = (oldData: Device.ActiveDevice[]) => {
  return oldData.map((activeDevice) => {
    const newActiveDevice: Device.ActiveDevice = {
      uid: activeDevice.uid,
      type: activeDevice.type,
      data: JSON.parse((activeDevice.data as unknown) as string),
      ip: activeDevice.ip,
    };
    return newActiveDevice;
  });
};

export const addRequest = (request: Client.Request) => {
  RequestModel.create(parseRequest(request));
};

export const getAllActiveDevices = async (): Promise<Device.ActiveDevice[]> => {
  const databaseData = await DeviceModel.find({});
  const parsedDbData = parseData(
    (databaseData as unknown) as Device.ActiveDevice[],
  );
  console.log({ parsedDbData });
  return parsedDbData;
};
