const { DEVICE_UID } = process.env;
if (!DEVICE_UID) throw new Error("DEVICE_UID not defined in .env");
export const EVENT_REQUEST_TOPIC = `${process.env.DEVICE_UID}/event/toDevice`;
export const ON_CONNECTED_TOPIC = 'on/connected';