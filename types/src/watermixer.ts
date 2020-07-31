export interface WatermixerData {
  remainingSeconds: number;
  isTimerOn: number; // 1 or 0
}

export const watermixerSampleData: WatermixerData = {
  remainingSeconds: 200,
  isTimerOn: 0,
}
