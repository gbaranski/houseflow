import { Topic } from '.';

export namespace Watermixer {
  export interface Data {
    finishMixTimestamp: number;
  }

  export const SAMPLE: Data = {
    finishMixTimestamp: 0,
  };

  export const getStartMixingTopic = (uid: string): Topic => ({
    request: `${uid}/event/relay1/request`,
    response: `${uid}/event/relay1/response`,
  });
}
