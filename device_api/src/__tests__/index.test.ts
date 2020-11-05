import * as sinon from 'sinon';
import * as admin from 'firebase-admin';
import { v4 as uuidv4 } from 'uuid';
import supertest from 'supertest';
import { Client, Exceptions } from '@houseflow/types';

if (process.env.CI) {
  admin.initializeApp({
    credential: admin.credential.cert(process.env.FIREBASE_CERT as string),
    databaseURL: 'https://houseflow-dev.firebaseio.com',
  });
} else {
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const serviceAccount = require('./firebaseConfig.json');
  admin.initializeApp({
    credential: admin.credential.cert(serviceAccount),
    databaseURL: 'https://houseflow-dev.firebaseio.com',
  });
}

process.env.DEVICE_API_USERNAME = uuidv4();
process.env.DEVICE_API_PASSWORD = uuidv4();

describe('POST /request', () => {
  let adminStub: any;
  let firebaseStub: any;
  let mqttClientStub: any;
  let mqttServiceStub: any;
  let api: any;
  let firebaseFile: any;
  const firebaseUser: Client.FirebaseUser = {
    devices: [],
    role: 'user',
    uid: uuidv4(),
    username: uuidv4(),
  };

  let usersCollectionListener: () => void;
  beforeAll(async (done) => {
    adminStub = sinon.stub(admin, 'initializeApp');

    // eslint-disable-next-line @typescript-eslint/no-var-requires
    firebaseFile = require('../services/firebase');
    firebaseStub = sinon.stub(firebaseFile, 'decodeToken').resolves({
      uid: firebaseUser.uid,
      aud: 'houseflow-dev',
    } as admin.auth.DecodedIdToken);
    firebaseStub = sinon.stub(firebaseFile, 'addRequestHistory').resolves();

    usersCollectionListener = firebaseFile.usersCollectionListener;

    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const mqttClient = require('../services/mqttClient');
    const fakeMqttClient: any = {
      connected: true,
    };

    mqttClientStub = sinon
      .stub(mqttClient, 'createMqttClient')
      .returns(fakeMqttClient);

    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const mqttService = require('../services/mqtt');
    mqttServiceStub = sinon
      .stub(mqttService, 'sendDeviceMessage')
      .resolves(Exceptions.SUCCESS);

    // eslint-disable-next-line @typescript-eslint/no-var-requires
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    api = supertest(require('../app').app);
    done();
  });

  afterAll(() => {
    adminStub.restore();
    firebaseStub.restore();
    mqttClientStub.restore();
    mqttServiceStub.restore();
    usersCollectionListener();
  });

  beforeEach(() => {
    firebaseFile.firebaseUsers = [];
    firebaseUser.devices = [];
  });

  it('Empty body', async () => {
    const res = await api.post('/request').send();
    expect(res.status).toEqual(400);
  });
  it('Attempting not existing user', async () => {
    const req: Client.DeviceRequest = {
      user: {
        token: uuidv4(),
      },
      device: {
        uid: uuidv4(),
        gpio: 1,
        action: 'toggle',
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(403);
  });
  it('Attempting with existing user, but without permission to device', async () => {
    firebaseFile.firebaseUsers = [firebaseUser];
    const req: Client.DeviceRequest = {
      user: {
        token: 'itCanBeAnything',
      },
      device: {
        uid: uuidv4(),
        gpio: 1,
        action: 'toggle',
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(403);
  });
  it('Attempting with existing user, without execute access', async () => {
    const firebaseUserDevice: Client.FirebaseUserDevice = {
      uid: uuidv4(),
      read: true,
      write: true,
      execute: false,
    };
    firebaseUser.devices = [firebaseUserDevice];

    firebaseFile.firebaseUsers = [firebaseUser];
    const req: Client.DeviceRequest = {
      user: {
        token: 'itCanBeAnything',
      },
      device: {
        uid: firebaseUserDevice.uid,
        gpio: 1,
        action: 'toggle',
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(403);
  });
  it('Attempting with existing user, with execute access to device', async () => {
    const firebaseUserDevice: Client.FirebaseUserDevice = {
      uid: uuidv4(),
      read: false,
      write: false,
      execute: true,
    };
    firebaseUser.devices = [firebaseUserDevice];

    firebaseFile.firebaseUsers = [firebaseUser];
    const req: Client.DeviceRequest = {
      user: {
        token: 'itCanBeAnything',
      },
      device: {
        uid: firebaseUserDevice.uid,
        gpio: 1,
        action: 'toggle',
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(200);
  });
});
