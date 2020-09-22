import { MqttClient } from 'mqtt';

export default () => {
  const mixWater = (uid: string, mqttClient: MqttClient) => {
    console.log('Mixing water');
    console.log(Date.now());

    mqttClient.publish(`${uid}/event/startmix`, '');
  };

  return {
    mixWater,
  };
};
