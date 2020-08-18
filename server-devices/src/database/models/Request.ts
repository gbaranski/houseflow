import mongoose, { model } from 'mongoose';
import { REQUEST } from '.';

const RequestSchema = new mongoose.Schema({
  requestType: String,
  deviceUid: String,
  deviceType: String,
  data: String,
});

export default model(REQUEST, RequestSchema);
