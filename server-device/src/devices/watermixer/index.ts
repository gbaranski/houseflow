import Device from '../';
import {
  Device as DeviceType, Client, Watermixer,
} from '@gbaranski/types';
import { MqttClient } from 'mqtt';
import { publishDeviceData } from '@/services/redis_pub';
import { startMixingTopic } from '@/topics';

class WatermixerDevice extends Device<Watermixer.Data> {
  private static MIXER_TIMEOUT_SECONDS = 600;

  private mixerInterval: NodeJS.Timeout | undefined;


  constructor(
    mqttClient: MqttClient,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<Watermixer.Data>,
  ) {
    super(mqttClient, firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void { // TODO fix later
    console.log({ message });
  }

  private startMixing() {
    if (this.mixerInterval) clearInterval(this.mixerInterval);
    this.activeDevice.data.isTimerOn = true;
    this.activeDevice.data.remainingSeconds = WatermixerDevice.MIXER_TIMEOUT_SECONDS;

    const decrementValues = () => {
      this.activeDevice.data.remainingSeconds -= 1;
      publishDeviceData(this.activeDevice);
      if (this.activeDevice.data.remainingSeconds < 1) {

        this.activeDevice.data.isTimerOn = false;
        this.activeDevice.data.remainingSeconds = 0;
        publishDeviceData(this.activeDevice);

        if (this.mixerInterval) clearInterval(this.mixerInterval);
      }
    }

    decrementValues();

    this.mixerInterval = setInterval(() => {
      decrementValues();
    }, 1000)

  }

  public requestDevice(request: Client.Request) {
    const requestData = {
      type: request.requestType,
      data: request.data,
    };
    console.log('Sending', requestData, `to ${this.firebaseDevice.uid}`);

    if (request.requestType === 'START_MIXING') {
      this.mqttClient.publish(startMixingTopic(this.firebaseDevice.uid), "");
      console.log("Starting mixing water");
      this.startMixing();
    }

    return true;
  }

}

export default WatermixerDevice;
