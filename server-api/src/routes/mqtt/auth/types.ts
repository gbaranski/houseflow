export interface UserRequest {
  clientid: string;
  ip: string;
  username: string;
  password: string;
}

export enum MqttAccess {
  'sub' = 1,
  'pub' = 2,
}
export interface AclRequest {
  access: MqttAccess;
  clientid: string;
  ip: string;
  username: string;
  topic: string;
}
