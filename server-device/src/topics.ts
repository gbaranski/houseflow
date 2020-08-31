import { Device } from "@gbaranski/types";

export const getEventTopic = (req: Device.RequestDevice) => `${req.topic.uid}/event/todevice/${req.topic.name}`;
export const ON_CONNECTED_TOPIC = 'on/connected';