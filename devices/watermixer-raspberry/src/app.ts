import { MqttClient } from "mqtt";
import { EVENT_REQUEST_TOPIC, ON_CONNECTED_TOPIC } from "./topics";

const { DEVICE_UID, DEVICE_SECRET, MQTT_URL } = process.env;

export const onConnection = (mqtt: MqttClient) => {
    console.log("Initialized connection with MQTT");
    subscribeToTopics(mqtt);
    mqtt.on('message', onMessage);
    publishOnConnectedMessage(mqtt);
}

const publishOnConnectedMessage = (mqtt: MqttClient) => {
    const message = {
        uid: DEVICE_UID,
        secret: DEVICE_SECRET,
    }
    mqtt.publish(ON_CONNECTED_TOPIC, JSON.stringify(message));
}

const subscribeToTopics = (mqtt: MqttClient) => {
    mqtt.subscribe(EVENT_REQUEST_TOPIC);
}

export const onMessage = (topic: String, message: Buffer) => {
    console.log({ topic, message });
    switch (topic) {
        case EVENT_REQUEST_TOPIC:
            console.log("Received request on EVENT_REQUEST_TOPIC");
            break;
        default:
            console.log("Unrecognized topic");
            break;
    }
}