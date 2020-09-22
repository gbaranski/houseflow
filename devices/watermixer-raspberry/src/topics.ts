const { DEVICE_UID } = process.env;
if (!DEVICE_UID) throw new Error('DEVICE_UID not defined in .env');
export const START_MIX_TOPIC = `${DEVICE_UID}/event/startmix`;
export const ON_CONNECTED_TOPIC = 'on/connected';
