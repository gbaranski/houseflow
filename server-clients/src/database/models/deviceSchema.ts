import mongoose, { model } from 'mongoose';
import { DEVICE } from '.';

const DeviceSchema = new mongoose.Schema({
  uid: {
    type: String,
    required: true,
    unique: true,
  },
  data: {
    type: mongoose.Schema.Types.ObjectId,
    required: true,
  },
  ip: {
    type: String,
    required: true,
  },
  type: {
    type: String,
    required: true,
  },
});

export default model(DEVICE, DeviceSchema);
