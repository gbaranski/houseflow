import { Topic } from '.';

export namespace Gate {
  export interface Data {
    lastOpenTimestamp: number;
  }

  export const SAMPLE: Data = {
    lastOpenTimestamp: 0,
  };

  export const getOpenGateTopic = (uid: string): Topic => ({
    request: `${uid}/event/gate/open/request`,
    response: `${uid}/event/gate/open/response`,
  });
}
