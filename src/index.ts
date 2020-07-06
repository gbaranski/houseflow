/* eslint-disable no-console */
import express from 'express';
import cors from 'cors';
import './firebase';
import appRouter from './app';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
const httpPort = 8000;

const app = express();

const whitelist = [
  'https://control.gbaranski.com',
  'http://localhost:3000',
  '*',
];

app.use(cors({ origin: whitelist }));
app.use(express.json()); // for parsing application/json

app.use(appRouter);

app.listen(httpPort, (): void =>
  console.log(`API-Server listening at http://localhost:${httpPort}`),
);
