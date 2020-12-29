import {
  SmartHomeV1SyncDeviceInfo,
  SmartHomeV1SyncName,
} from 'actions-on-google';
import { Collection, Db, MongoClient, ObjectId } from 'mongodb';

const { MONGO_INITDB_ROOT_USERNAME, MONGO_INITDB_ROOT_PASSWORD } = process.env;
if (!MONGO_INITDB_ROOT_USERNAME || !MONGO_INITDB_ROOT_PASSWORD)
  throw new Error(
    'MONGO_INITDB_ROOT_USERNAME or MONGO_INITDB_ROOT_PASSWORD are missing in .env',
  );

const DB_NAME = process.env.DB_NAME || 'houseflowDB';

const client = new MongoClient('mongodb://mongo:27017', {
  auth: {
    user: MONGO_INITDB_ROOT_USERNAME,
    password: MONGO_INITDB_ROOT_PASSWORD,
  },
  useUnifiedTopology: true,
});

let db: Db;
let devicesCollection: Collection;
let usersCollection: Collection;

export const connectMongo = async (): Promise<void> => {
  console.log('Attempting connection to mongoDB');
  await client.connect();
  console.log('Successfully connected to mongoDB');
  db = client.db(DB_NAME);
  devicesCollection = db.collection('devices');
  usersCollection = db.collection('users');
};

interface BaseDeviceData {
  online: boolean;
}

interface WaterHeaterData extends BaseDeviceData {
  isRunning: boolean;
}

interface GarageData extends BaseDeviceData {
  openPercent: number;
}

interface LightData extends BaseDeviceData {
  on: boolean;
}

export type AnyDeviceData = WaterHeaterData | GarageData | LightData;

export interface Device {
  // defined only when retreiving from DB
  _id?: string;

  type: string;
  traits: string[];
  name: SmartHomeV1SyncName;
  willReportState: boolean;
  deviceInfo: SmartHomeV1SyncDeviceInfo;
  roomHint: string;
  data: AnyDeviceData;
  attributes?: object;
}

export const addDevice = async (device: Device): Promise<void> => {
  const res = await devicesCollection.insertOne(device);
};

export const findDevices = async (deviceIDs: string[]): Promise<Device[]> => {
  const objectIDs = deviceIDs.map((id) => new ObjectId(id));
  const res = devicesCollection.find({
    _id: {
      $in: objectIDs,
    },
  });
  return res.toArray();
};

export interface User {
  _id?: string;

  firstName: string;
  lastName: string;
  email: string;
  password: string;
  devices: string[];
}

export const getUser = async (userID: string): Promise<User> => {
  const user = await usersCollection.findOne({
    _id: new ObjectId(userID),
  });
  if (!user) throw new Error('Unable to find user with matching ID');
  return user;
};
