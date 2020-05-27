import { keys } from './private/keys';

export function isAuthenticated(key: string) {
  return keys.authKeys.includes(key);
}
