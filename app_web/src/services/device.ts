import { Client } from '@houseflow/types';
import axios from 'axios';

const DEVICE_API_URL = 'http://localhost/device';

export const sendDeviceRequest = async (request: Client.DeviceRequest) => {
  console.log(request);
  return axios.post(`${DEVICE_API_URL}/request`, request);
};
