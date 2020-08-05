export const CORS_WHITELIST = [
  'https://control.gbaranski.com',
  'http://localhost:3000',
];

export const LOGIN_WHITELIST_URL = [
  '/',
  '/api/getToken',
  '/api/getDeviceToken',
];
export const NO_LOG_URL = [
  '/api/getDeviceToken',
  '/api/login',
  '/',
  '/favicon.ico',
  '/api/getDeviceStatus',
  '/watermixer/getData',
  '/alarmclock/getData',
];
