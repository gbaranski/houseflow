import mongoose, { model } from 'mongoose';
import { DEVICE } from '.';

const DeviceScheme = new mongoose.Schema({
  uid: {
    type: String,
    required: true,
    unique: true,
  },
  data: {
    // ALARMCLOCK
    alarmTime: {
      hour: mongoose.SchemaTypes.Number,
      minute: mongoose.SchemaTypes.Number,
      second: mongoose.SchemaTypes.Number,
    },
    alarmState: mongoose.SchemaTypes.Boolean,
    sensor: {
      temperature: mongoose.SchemaTypes.Number,
      humidity: mongoose.SchemaTypes.Number,
      heatIndex: mongoose.SchemaTypes.Number,
    },
    // WATERMIXER
    remainingTime: mongoose.SchemaTypes.Number,
    isTimerOn: mongoose.SchemaTypes.Boolean,
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

export default model(DEVICE, DeviceScheme);
