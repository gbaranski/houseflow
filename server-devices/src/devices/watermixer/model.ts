import mongoose, { model } from 'mongoose';

const WatermixerSchema = new mongoose.Schema({
  uid: {
    type: String,
    required: true,
    unique: true,
  },
  data: {
    remainingTime: mongoose.Schema.Types.Number,
    isTimerOn: mongoose.Schema.Types.Boolean,
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

export default model('Watermixer', WatermixerSchema);
