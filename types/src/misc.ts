export interface DateTime {
  hour: number;
  minute: number;
  second: number;
}

export interface TempHistory {
  unixTime: number;
  temperature: number;
}

export interface RequestHistory {
  user: string;
  requestPath: string;
  unixTime: number;
  ip: string;
  userAgent: string;
  country: string;
}
