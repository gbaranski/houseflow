import socketio from 'socket.io';
import { decodeClientToken, DocumentReference } from '@/services/firebase';
import { admin } from 'firebase-admin/lib/auth';
import { Client, Device } from '@gbaranski/types';

export const verifyClient = async (
  socket: socketio.Socket,
): Promise<admin.auth.DecodedIdToken> => {
  const { token } = socket.request;
  if (!token) throw new Error('Token is not defined');
  if (typeof token !== 'string') throw new Error('Token is not type of string');
  return await decodeClientToken(token);
};

export const joinDeviceChannels = async (
  firebaseUser: Client.FirebaseUser,
  client: socketio.Socket,
) => {
  firebaseUser.devices.forEach(async (deviceRef: DocumentReference) => {
    const snapshot = await deviceRef.get();
    const device = snapshot.data() as Device.FirebaseDevice;
    if (!device.uid)
      throw new Error('There was an error with retreiving device uid');
    client.join(device.uid);
    console.log(`${firebaseUser.uid} joined ${device.uid}`);
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
