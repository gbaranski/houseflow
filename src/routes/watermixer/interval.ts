import fetch from 'node-fetch';
import { WATERMIXER_URL } from '../../config';
import { getProcessing } from '../globals';
import { WatermixerData, WaterRequestType } from '@gbaranski/types';
import { setProcessingWatermixer } from '.';

let data: WatermixerData;

export async function watermixerInterval(): Promise<void> {
  if (getProcessing().watermixer) {
    console.log('Connection overloaded at watermixer');
    return;
  }
  setProcessingWatermixer(true);
  fetch(WATERMIXER_URL + WaterRequestType.GET_DATA)
    .then((res): Promise<WatermixerData> => res.json())
    .then((_data): void => {
      data = _data;
      console.log('Fetched watermixer data');
    })
    .catch((e): void => {
      console.log(e);
    })
    .finally((): void => {
      setProcessingWatermixer(false);
    });
}

export function getData(): WatermixerData {
  return data;
}
