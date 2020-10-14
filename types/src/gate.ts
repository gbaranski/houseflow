import { Topic } from '.';

export namespace Gate {
  export interface Data {
    lastOpenTimestamp: number;
  }

  export const SAMPLE: Data = {
    lastOpenTimestamp: 0,
  };

  export const getOpenGateTopic = (uid: string): Topic => ({
    request: `${uid}/event/open/request`,
    response: `${uid}/event/open/response`,
  });
}
