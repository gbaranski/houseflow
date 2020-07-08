import express from 'express';

function validateCredentials(
  username: string | undefined,
  password: string | undefined,
) {
  if (!username || !password) {
    throw new Error(`Invalid username/password`);
  }
  if (process.env[username.toUpperCase()] !== password) {
    throw new Error(`Invalid username/password`);
  }
}

export function isAuthenticated(
  req: express.Request,
  res: express.Response,
  next: express.NextFunction,
): void {
  const username = req.header('username');
  const password = req.header('password');
  validateCredentials(username, password);
  next();
}
