import socketio from 'socket.io';
import { decodeClientToken, DocumentReference } from '@/services/firebase';
import { admin } from 'firebase-admin/lib/auth';
import { Client, Device, Watermixer } from '@gbaranski/types';

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

export const setupEventListeners = (socket: socketio.Socket, uid: string) => {
  socket.on('event', (data) => {
    console.log(`${uid} sent event`);
    console.log(data);
  });
  socket.on('disconnect', () => {
    console.log(`${uid} disconnected`);
  });
};

export const publishDeviceData = (
  socket: socketio.Server,
  device: Device.ActiveDevice,
) => {
  console.log('Publishing device data over socket');
  socket.to(device.uid).emit('device_data', JSON.stringify(device));
};
