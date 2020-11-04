import { v4 as uuidv4 } from 'uuid';
// To prevent erros with env variables
process.env.DEVICE_API_USERNAME = 'dontCareAboutIt';
process.env.DEVICE_API_PASSWORD = 'dontCareAboutIt';
import app from '../app';
import request from 'supertest';
import { AclRequest, UserRequest } from '@/routes/mqtt/types';
import {
  devicePrivateCollection,
  PrivateDeviceData,
} from '@/services/firebase';

const randomId = () => Math.random().toString(16).substr(2, 8);

const deviceUuid = uuidv4();
const privateDeviceData: PrivateDeviceData = { secret: uuidv4() };

const initDatabase = () => {
  return devicePrivateCollection.doc(deviceUuid).set(privateDeviceData);
};
const clearDatabase = () => {
  return devicePrivateCollection.doc(deviceUuid).delete();
};

beforeAll(async () => {
  await initDatabase();
});

afterAll(async () => {
  await clearDatabase();
});

describe('POST /mqtt/user', () => {
  beforeEach(() => {
    jest.spyOn(console, 'log').mockImplementation(() => {});
  });

  it('sending no credentials', async () => {
    const res = await request(app).post('/mqtt/user').send();
    expect(res.status).toEqual(400);
  });
  it('sending invalid credentials', async () => {
    const userRequest: UserRequest = {
      clientid: randomId(),
      ip: randomId(),
      username: randomId(),
      password: randomId(),
    };
    const res = await request(app).post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(400);
  });
  it('sending invalid credentials with schematiccly valid data', async () => {
    const userRequest: UserRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: 'somethingWhatIs36CharactersLength123',
      password: 'somethingWhatIs36CharactersLength123',
    };
    const res = await request(app).post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(400);
  });
  it('sending valid credentials', async () => {
    const userRequest: UserRequest = {
      clientid: `device_${randomId()}`,
      ip: '1.1.1.1',
      username: deviceUuid,
      password: privateDeviceData.secret,
    };
    const res = await request(app).post('/mqtt/user').send(userRequest);
    expect(res.status).toEqual(200);
  });
});

describe('POST /mqtt/acl', () => {
  //   beforeEach(() => {
  //     jest.spyOn(console, 'log').mockImplementation(() => {});
  //   });

  it('sending no credentials', async () => {
    const res = await request(app).post('/mqtt/acl').send();
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
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
    const res = await request(app).post('/mqtt/acl').send(userRequest);
    expect(res.status).toEqual(200);
  });
});
