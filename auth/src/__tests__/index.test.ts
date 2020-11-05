import { v4 as uuidv4 } from 'uuid';
// To prevent erros with env variables
process.env.DEVICE_API_USERNAME = 'dontCareAboutIt';
process.env.DEVICE_API_PASSWORD = 'dontCareAboutIt';
import { AclRequest, UserRequest } from '@/routes/mqtt/types';
import admin from 'firebase-admin';
import sinon from 'sinon';
import supertest from 'supertest';

if (process.env.CI) {
  admin.initializeApp({
    credential: admin.credential.cert(process.env.FIREBASE_CERT as string),
    databaseURL: 'https://houseflow-dev.firebaseio.com',
  });
} else {
  const serviceAccount = require('./firebaseConfig.json');
  admin.initializeApp({
    credential: admin.credential.cert(serviceAccount),
    databaseURL: 'https://houseflow-dev.firebaseio.com',
  });
}

const randomId = () => Math.random().toString(16).substr(2, 8);

const deviceUuid = uuidv4();
const privateDeviceData: { secret: string } = { secret: uuidv4() };

describe('POST /mqtt/user', () => {
  let adminStub: any;
  let firebaseFile: any;
  let api: any;
  beforeAll(async (done) => {
    adminStub = sinon.stub(admin, 'initializeApp');
    firebaseFile = require('../services/firebase');
    await admin
      .firestore()
      .collection('devices-private')
      .doc(deviceUuid)
      .set(privateDeviceData);

    api = supertest(require('../app').app);
    console.log(api.address);
    done();
  });
  afterAll(async (done) => {
    adminStub.restore();
    await admin
      .firestore()
      .collection('devices-private')
      .doc(deviceUuid)
      .delete();
    done();
  });

  it('sending no credentials', async () => {
    const res = await api.post('/mqtt/user').send();
    expect(res.status).toEqual(400);
  });
  it('sending invalid credentials', async () => {
    const userRequest: UserRequest = {
      clientid: randomId(),
      ip: randomId(),
      username: randomId(),
      password: randomId(),
    };
    const res = await api.post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(400);
  });
  it('sending invalid credentials with schematiccly valid data', async () => {
    const userRequest: UserRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: 'somethingWhatIs36CharactersLength123',
      password: 'somethingWhatIs36CharactersLength123',
    };
    const res = await api.post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(400);
  });
  it('sending valid credentials', async () => {
    const userRequest: UserRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      password: privateDeviceData.secret,
    };
    const res = await api.post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(200);
  });
});

describe('POST /mqtt/acl', () => {
  let adminStub: any;
  let firebaseFile: any;
  let api: any;
  beforeAll(() => {
    firebaseFile = require('../services/firebase');
    adminStub = sinon.stub(admin, 'initializeApp');
    api = supertest(require('../app').app);
  });
  afterAll(async (done) => {
    adminStub.restore();
    done();
  });

  beforeEach(() => {
    jest.spyOn(console, 'log').mockImplementation(() => {});
  });

  it('sending no credentials', async () => {
    const res = await api.post('/mqtt/acl').send();
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with # wildcard', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `${deviceUuid}/#`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with + wildcard', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `${deviceUuid}/+`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with $ SYS Topic', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `$something`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with response suffix ', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `${deviceUuid}/trigger/light1/response`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL publish with request suffix ', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '2',
      topic: `${deviceUuid}/trigger/light1/request`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL publish with no device_ suffix ', async () => {
    const userRequest: AclRequest = {
      clientid: `mobile_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `${deviceUuid}/trigger/light1/request`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with no device_ suffix ', async () => {
    const userRequest: AclRequest = {
      clientid: `mobile_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '2',
      topic: `${deviceUuid}/trigger/light1/request`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL subscribe with mismatching with username prefix', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: uuidv4(),
      access: '1',
      topic: `${deviceUuid}/trigger/light1/request`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending ACL publish with mismatching with username prefix', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: uuidv4(),
      access: '2',
      topic: `${deviceUuid}/trigger/light1/response`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(403);
  });
  it('sending valid ACL publish', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '2',
      topic: `${deviceUuid}/trigger/light1/response`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(200);
  });
  it('sending valid ACL subscribe', async () => {
    const userRequest: AclRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      access: '1',
      topic: `${deviceUuid}/trigger/light1/request`,
    };
    const res = await api.post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(200);
  });
});
