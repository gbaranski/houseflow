export interface UserRequest {
  clientid: string;
  ip: string;
  username: string;
  password: string;
}

export interface AclRequest {
  access: '1' | '2'; // 1 = sub, 2 = pub
  clientid: string;
  ip: string;
  username: string;
  topic: string;
}
