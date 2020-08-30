import Device from '../';
import {
  Device as DeviceType,
} from '@gbaranski/types';
import { MqttClient } from 'mqtt';

class WatermixerDevice extends Device {
  constructor(
    mqttClient: MqttClient,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice,
  ) {
    super(mqttClient, firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void { // TODO fix later
    console.log({ message });
  }
}

export default WatermixerDevice;
