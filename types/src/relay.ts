import { Topic } from '.';

export namespace Relay {
  export interface Data {
    lastSignalTimestamp: number;
  }

  export const SAMPLE: Data = {
    lastSignalTimestamp: 0,
  };

  export const getSendSignalTopic = (uid: string): Topic => ({
    request: `${uid}/event/relay1/request`,
    response: `${uid}/event/relay1/response`,
  });
}
