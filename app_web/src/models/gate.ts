import { updateDeviceData } from '@/services/firebase';
import { sendRequest } from '@/services/mqtt';
import { getRandomShortUid } from '@/utils/utils';
import { Device, Gate } from '@gbaranski/types';
import { MqttClient } from 'mqtt';

export default () => {
  const openGate = (device: Device.FirebaseDevice, mqttClient: MqttClient) => {
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };

    const onSuccess = () => {
      const lastOpenTimestamp = Date.now();
      updateDeviceData(device.uid, {
        lastOpenTimestamp,
      });
    };

    sendRequest({
      request,
      topic: Gate.getOpenGateTopic(device.uid),
      mqttClient,
      onSuccess,
    });
  };

  return {
    openGate,
  };
};
