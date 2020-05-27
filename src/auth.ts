export function isAuthenticated(username: string, key: string) {
  return process.env[username.toUpperCase()] === key;
}
