import { updateDeviceData } from '@/services/firebase';
import { sendRequest } from '@/services/mqtt';
import { getRandomShortUid } from '@/utils/utils';
import { Device, Gate } from '@gbaranski/types';
import { message } from 'antd';
import { MqttClient } from 'mqtt';

export default () => {
  const openGate = async (device: Device.FirebaseDevice, mqttClient: MqttClient) => {
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };

    try {
      message.info('Sending!');
      await sendRequest({
        request,
        topic: Gate.getOpenGateTopic(device.uid),
        mqttClient,
      });
      updateDeviceData(device.uid, {
        lastOpenTimestamp: Date.now(),
      });
    } catch (e) {
      console.log(`Error when sending request ${e}`);
      message.error(e.message);
    }
  };

  return {
    openGate,
  };
};
