export enum DeviceTypes {
  ALARMCLOCK = 'alarmclock',
  WATERMIXER = 'watermixer',
}
export interface Credentials {
  username: string;
  password: string;
}

export const remoteUrl = 'https://api.gbaranski.com';
