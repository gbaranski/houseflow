import { Client } from '@houseflow/types';
import { message } from 'antd';
import axios from 'axios';
import { getGeoPoint } from './misc';

const DEVICE_API_URL = 'https://api.gbaranski.com/device';

type Overwrite<T1, T2> = {
  [P in Exclude<keyof T1, keyof T2>]: T1[P];
} &
  T2;

export const sendDeviceRequest = async (
  request: Overwrite<Client.DeviceRequest, { user: { token: string } }>,
): Promise<void> => {
  try {
    const geoPoint = await getGeoPoint();
    if (!geoPoint) return;
    const deviceRequest: Client.DeviceRequest = {
      ...request,
      user: {
        token: request.user.token,
        geoPoint,
      },
    };
    await axios.post<string>(`${DEVICE_API_URL}/request`, deviceRequest);

    message.success('Success!');
  } catch (e) {
    console.log(e.response);
    message.error(`${e.response.status} ${e.response.statusText} - ${e.response.data}`);
  }
};
