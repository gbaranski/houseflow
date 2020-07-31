export interface AlarmclockData {
  currentTime: string; // formatted HH/MM/SS
  alarmTime: string; // formatted HH/MM/SS
  remainingTime: string; // formatted HH/MM/SS
  alarmState: number; // 1 or 0
  temperature: number;
  humidity: number;
  heatIndex: number;
}

export interface AlarmclockHeaders {
  time: string; // formatted HH/MM/SS
  state: number; // 1 or 0
}

export const alarmclockSampleData: AlarmclockData = {
  currentTime: "00:00:00",
  alarmTime: "00:00:00",
  remainingTime: "00:00:00",
  alarmState: 0,
  temperature: 0,
  humidity: 0,
  heatIndex: 0,
}
