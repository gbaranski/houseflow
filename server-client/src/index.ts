import socketio from 'socket.io';
import chalk from 'chalk';
import {
  joinDeviceChannels,
  setupEventListeners,
  verifyClient,
} from '@/services/websocket';
import http from 'http';
import '@/services/redis_sub';
import '@/services/redis_pub';
import { convertToFirebaseUser } from './services/firebase';

const PORT = process.env.PORT_CLIENT;
if (!PORT) throw new Error('Port is not defined in .env');

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from client zone');
};

const httpServer = http.createServer(requestListener);

const io = socketio(httpServer, {});

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

  joinDeviceChannels(firebaseUser, socket);
  setupEventListeners(socket, firebaseUser.uid);
});

httpServer.listen(PORT, () =>
  console.log(chalk.yellow(`Listening for HTTP requests at ${PORT}`)),
);
