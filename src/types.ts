export enum AlarmRequestType {
  GET_DATA = '/getESPData',
  GET_TEMP_ARRAY = '/getTempArray',
  GET_DEVICE_STATE = '/isDown',
  SET_TIME = '/setAlarm',
  SWITCH_STATE = '/setAlarmState',
  TEST_ALARM = '/testAlarm',
}
export enum WaterRequestType {
  GET_DATA = '/getESPData',
  START_MIXING = '/startMixing',
}

export enum OtherRequestsType {
  GET_DEVICES_STATUS = '/getDevicesStatus',
}

export interface AlarmclockData {
  currentTime: string;
  alarmTime: string;
  remainingTime: string;
  alarmState: number;
  temperature: number;
  humidity: number;
  heatIndex: number;
}

export interface WatermixerData {
  remainingSeconds: string;
  isTimerOn: string;
}

export interface RequestHistory {
  user: string;
  requestType: AlarmRequestType | WaterRequestType;
  date: Date;
}
