export function isAuthenticated(
  username: string | undefined,
  password: string | undefined,
): boolean {
  if (!username || !password) {
    return false;
  }
  return process.env[username.toUpperCase()] === password;
}
