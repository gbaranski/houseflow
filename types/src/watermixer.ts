import WebSocket from 'ws';

export interface IWatermixer {
  data: IWatermixerData;
}

export interface IWatermixerData {
  remainingSeconds: number;
  isTimerOn: boolean; // 1 or 0
}

export const watermixerSample: IWatermixerData = {
  remainingSeconds: 0,
  isTimerOn: true,
};

export enum WatermixerRequestTypes {
  GET_DATA = 'GET_DATA',
  START_MIXING = 'START_MIXING',
  SET_TIME = 'SET_TIME',
  SET_STATE = 'SET_STATE',
  TEST_SIREN = 'TEST_SIREN',
  REBOOT = 'REBOOT',
  UNKNOWN = 'UNKNOWN',
}

export type TRequestWatermixer = ((
  type: WatermixerRequestTypes.GET_DATA,
) => any) &
  ((type: WatermixerRequestTypes.START_MIXING) => any) &
  ((type: WatermixerRequestTypes.REBOOT) => any);
