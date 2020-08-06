import express from 'express';
import http from 'http';

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
  try {
    validateCredentials(username, password);
  } catch (e) {
    throw new Error('No authorization');
  }
  next();
}

export interface VerifyInfo {
  origin: string;
  secure: boolean;
  req: http.IncomingMessage;
}

export interface VerifyCallback {
  (
    res: boolean,
    code?: number,
    message?: string,
    headers?: http.OutgoingHttpHeaders,
  ): void;
}
