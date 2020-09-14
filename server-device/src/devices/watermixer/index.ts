import Device from '../';
import { Device as DeviceType, Watermixer } from '@gbaranski/types';
import { MqttClient } from 'mqtt';
import { getEventTopic } from '@/topics';
import { publishDeviceData } from '@/services/gcloud';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

class WatermixerDevice extends Device<Watermixer.Data> {
  constructor(
    mqttClient: MqttClient,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<Watermixer.Data>,
  ) {
    super(mqttClient, firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void {
    // TODO fix later
    console.log({ message });
  }

  private startMixing() {
    this.activeDevice.data.finishMixTimestamp =
      Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;

    publishDeviceData(this.activeDevice);
  }

  public requestDevice(request: DeviceType.RequestDevice) {
    console.log('Sending', request, `to ${this.firebaseDevice.uid}`);

    if (request.topic.name === 'startmix') {
      this.mqttClient.publish(getEventTopic(request), '');
      console.log('Starting mixing water');
      this.startMixing();
    }

    return true;
  }
}

export default WatermixerDevice;
