import * as sinon from 'sinon';
import * as admin from 'firebase-admin';
// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('./firebaseConfig.json');

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
  databaseURL: 'https://houseflow-dev.firebaseio.com',
});

sinon.stub(admin, 'initializeApp');

import * as firebase from '../services/firebase';
sinon.stub(firebase, 'decodeToken').resolves({ uid: 'asdf' } as any);

import * as mqttClient from '../services/mqttClient';
const fakeMqttClient: any = {};

sinon.stub(mqttClient, 'createMqttClient').returns(fakeMqttClient);

describe('test index', () => {
  beforeAll(() => {});
  it('test', () => {});
});
