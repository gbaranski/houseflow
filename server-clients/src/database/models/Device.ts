import mongoose, { model } from 'mongoose';
import { DEVICE } from '.';

const DeviceScheme = new mongoose.Schema({
  uid: {
    type: String,
    required: true,
    unique: true,
  },
  data: String,
  ip: {
    type: String,
    required: true,
  },
  type: {
    type: String,
    required: true,
  },
});

export default model(DEVICE, DeviceScheme);
