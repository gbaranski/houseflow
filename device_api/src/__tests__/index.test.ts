import Sinon, * as sinon from 'sinon';
import * as admin from 'firebase-admin';
import { v4 as uuidv4 } from 'uuid';
import supertest from 'supertest';
import { Client, Device, Exceptions, GeoPoint } from '@houseflow/types';

if (process.env.CI) {
  admin.initializeApp({
    credential: admin.credential.cert(
      JSON.parse(process.env.FIREBASE_CERT as string),
    ),
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

const getRandomGeoPoint = (): GeoPoint => ({
  latitude: Math.random(),
  longitude: Math.random(),
});

describe('POST /request', () => {
  let adminStub: Sinon.SinonStub;
  let firebaseStub: Sinon.SinonStub;
  let mqttClientStub: Sinon.SinonStub;
  let mqttServiceStub: Sinon.SinonStub;
  let api: supertest.SuperTest<supertest.Test>;
  let firebaseFile: {
    decodeToken: (token: string) => Promise<admin.auth.DecodedIdToken>;
    firebaseUsers: Client.FirebaseUser[];
    usersCollectionListener: () => void;
    findDeviceByAction: (
      action: Device.Action,
      firebaseUser: Client.FirebaseUser,
    ) => Promise<Device.FirebaseDevice>;
    addRequestHistory: ({
      request,
      deviceUid,
      ipAddress,
      userUid,
    }: {
      request: Client.DeviceRequest;
      deviceUid: string;
      ipAddress: string;
      userUid: string;
    }) => Promise<void>;
  };
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
    const fakeMqttClient: { connected: boolean } = {
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
        geoPoint: getRandomGeoPoint(),
        token: uuidv4(),
      },
      device: {
        uid: uuidv4(),
        action: {
          name: Device.ActionName.MixWater, // anything
          id: 1,
        },
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(403);
  });
  it('Attempting with existing user, but without permission to device', async () => {
    firebaseFile.firebaseUsers = [firebaseUser];
    const req: Client.DeviceRequest = {
      user: {
        geoPoint: getRandomGeoPoint(),
        token: 'itCanBeAnything',
      },
      device: {
        uid: uuidv4(),
        action: {
          name: Device.ActionName.MixWater, // anything
          id: 1,
        },
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
        geoPoint: getRandomGeoPoint(),
        token: 'itCanBeAnything',
      },
      device: {
        uid: firebaseUserDevice.uid,
        action: {
          name: Device.ActionName.MixWater, // anything
          id: 1,
        },
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
        geoPoint: getRandomGeoPoint(),
        token: 'itCanBeAnything',
      },
      device: {
        uid: firebaseUserDevice.uid,
        action: {
          name: Device.ActionName.MixWater, // anything
          id: 1,
        },
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(200);
  });
  it('Attempting with existing user, but without UID', async () => {
    const firebaseUserDevice: Client.FirebaseUserDevice = {
      uid: uuidv4(),
      read: false,
      write: false,
      execute: true,
    };
    firebaseUser.devices = [firebaseUserDevice];
    sinon.stub(firebaseFile, 'findDeviceByAction').resolves({
      uid: firebaseUserDevice.uid,
      geoPoint: getRandomGeoPoint(),
      status: true,
      ip: 'unknown',
      data: undefined,

      type: Device.DeviceType.WATERMIXER, // that must match with actions property
      actions: [
        {
          name: Device.ActionName.MixWater,
          id: 1,
        },
      ],
    });

    firebaseFile.firebaseUsers = [firebaseUser];
    const req: Client.DeviceRequest = {
      user: {
        geoPoint: getRandomGeoPoint(),
        token: 'itCanBeAnything',
      },
      device: {
        action: {
          name: Device.ActionName.MixWater, // anything
          id: 1,
        },
      },
    };
    const res = await api.post('/request').send(req);
    expect(res.status).toEqual(200);
  });
});
