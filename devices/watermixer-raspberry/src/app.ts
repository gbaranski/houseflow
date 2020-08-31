import { MqttClient } from "mqtt";
import { ON_CONNECTED_TOPIC, START_MIX_TOPIC } from "./topics";
import { startMixing } from "./services/relay";

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
    mqtt.subscribe(START_MIX_TOPIC);
}

export const onMessage = (topic: String, message: Buffer) => {
    console.log({ topic, message });
    switch (topic) {
        case START_MIX_TOPIC:
            startMixing();
            break;
        default:
            console.log("Unrecognized topic");
            break;
    }
}
