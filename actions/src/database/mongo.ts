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

export const connectMongo = async (): Promise<void> => {
  console.log('Attempting connection to mongoDB');
  await client.connect();
  console.log('Successfully connected to mongoDB');
  db = client.db(DB_NAME);
  devicesCollection = db.collection('devices');
};

export interface DeviceData {
  on: boolean;
  online: boolean;
}
export interface Device {
  _id?: string;
  data: DeviceData;
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
