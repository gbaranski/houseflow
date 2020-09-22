const { DEVICE_UID } = process.env;
if (!DEVICE_UID) throw new Error('DEVICE_UID not defined in .env');
export const START_MIX_TOPIC_REQUEST = `${DEVICE_UID}/event/startmix/request`;
export const START_MIX_TOPIC_RESPONSE = `${DEVICE_UID}/event/startmix/response`;
export const ON_CONNECTED_TOPIC = 'on/connected';
