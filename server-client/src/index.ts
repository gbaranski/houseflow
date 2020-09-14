import socketio from 'socket.io';
import chalk from 'chalk';
import {
  joinDeviceChannels,
  setupEventListeners,
  verifyClient,
} from '@/services/websocket';
import http from 'http';
import { convertToFirebaseUser, DocumentReference } from './services/firebase';
import { subscribeToDevicesData } from './services/gcloud';
import { Device } from '@gbaranski/types';

const PORT = process.env.PORT_CLIENT;
if (!PORT) throw new Error('Port is not defined in .env');

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from client zone');
};

subscribeToDevicesData();
const httpServer = http.createServer(requestListener);

export const io = socketio(httpServer, {});

io.use(async (socket, next) => {
  try {
    await verifyClient(socket);
    next();
  } catch (e) {
    console.log(`${e.message} at Socket.IO client`);
    socket.error(e.message);
    socket.disconnect(true);
  }
});

io.on('connection', async (socket) => {
  console.log('Someone is connecting');

  const decodedClient = await verifyClient(socket);
  const firebaseUser = await convertToFirebaseUser(decodedClient.uid);

  const firebaseDevices = await Promise.all(
    firebaseUser.devices.map(
      async (ref: DocumentReference) =>
        (await ref.get()).data() as Device.ActiveDevice,
    ),
  );
  joinDeviceChannels(firebaseDevices, firebaseUser.uid, socket);
  setupEventListeners(socket, firebaseUser.uid, firebaseDevices);
});

httpServer.listen(PORT, () =>
  console.log(chalk.yellow(`Listening for HTTP requests at ${PORT}`)),
);
