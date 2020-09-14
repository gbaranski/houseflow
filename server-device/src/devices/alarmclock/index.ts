import Device from '../';
import { Device as DeviceType, Alarmclock } from '@gbaranski/types';

class AlarmclockDevice extends Device<Alarmclock.Data> {
  constructor(
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<Alarmclock.Data>,
  ) {
    super(firebaseDevice, activeDevice);
  }

  public handleMessage(message: any): void {
    // TODO fix later
    console.log({ message });
  }

  public handleRequest(request: DeviceType.RequestDevice) {
    // const requestData = {
    //   type: request.requestType,
    //   data: request.data,
    // };
    // console.log('Sending', requestData, `to ${this.firebaseDevice.uid}`);
    // // this.mqttClient.publish(getRequestTopic(this.firebaseDevice.uid), request.requestType);
    // console.log("Not implemeented sending");

    return true;
  }
}

export default AlarmclockDevice;
