import Device from '../';
import { Device as DeviceType, Watermixer } from '@gbaranski/types';
import { publishDeviceData } from '@/services/gcloud';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

class WatermixerDevice extends Device<Watermixer.Data> {
  constructor(
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<Watermixer.Data>,
  ) {
    super(firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void {
    //TODO: implement if needed
    console.log({ message });
  }

  private async startMixing() {
    this.activeDevice.data.finishMixTimestamp =
      Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;
    publishDeviceData(this._activeDevice);
  }

  public async handleRequest(request: DeviceType.RequestDevice) {
    if (request.topic.name === 'startmix') {
      console.log('Starting mixing water');
      this.startMixing();
    }
  }
}

export default WatermixerDevice;
