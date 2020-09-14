import socketio from 'socket.io';
import { decodeClientToken } from '@/services/firebase';
import { admin } from 'firebase-admin/lib/auth';
import { Device } from '@gbaranski/types';
import { activeDevices } from './gcloud';

export const verifyClient = async (
  socket: socketio.Socket,
): Promise<admin.auth.DecodedIdToken> => {
  const { token } = socket.handshake.query;
  if (!token) throw new Error('Token is not defined');
  if (typeof token !== 'string') throw new Error('Token is not type of string');
  return await decodeClientToken(token);
};

export const joinDeviceChannels = async (
  devices: Device.FirebaseDevice[],
  clientUid: string,
  client: socketio.Socket,
) => {
  devices.forEach(async (device) => {
    client.join(device.uid);
    console.log(`${clientUid} joined ${device.uid}`);
  });
};

export const setupEventListeners = (
  socket: socketio.Socket,
  uid: string,
  firebaseDevices: Device.FirebaseDevice[],
) => {
  socket.on('event', (data) => {
    console.log(`${uid} sent event`);
    console.log(data);
  });
  socket.on('disconnect', () => {
    console.log(`${uid} disconnected`);
  });
  socket.on('get_active_devices', (fn) => {
    const devices: Device.ActiveDevice[] = activeDevices.filter((device) =>
      firebaseDevices.some((_device) => _device.uid === device.uid),
    );
    fn(JSON.stringify(devices));
  });
};

export const updateDeviceData = (
  socket: socketio.Server,
  device: Device.ActiveDevice,
) => {
  console.log(`Publishing ${device.uid} data over socket`);
  socket.to(device.uid).emit('device_update', JSON.stringify(device));
};
