import { Topic } from '.';

export namespace Gate {
  export interface Data {
    lastOpenTimestamp: number;
  }

  export const SAMPLE: Data = {
    lastOpenTimestamp: 0,
  };

  export const getOpenGateTopic = (uid: string): Topic => ({
    request: `${uid}/event/relay1/request`,
    response: `${uid}/event/relay1/response`,
  });
}
