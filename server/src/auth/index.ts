import express from 'express';
import jwt from 'jsonwebtoken';
import http from 'http';
import { logMissing, logInvalid, logSocketAttempt } from '@/cli';

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

export function authenticateDevice(device: string, token: string): void {
  if (!token || !device) {
    throw new Error('No token or device name');
  }
  if (token !== process.env[device.toUpperCase()]) {
    throw new Error('Invalid token or device name');
  }
}

interface VerifyClientInfo {
  origin: string;
  secure: boolean;
  req: http.IncomingMessage;
}

interface VerifyClientCallback {
  (
    res: boolean,
    code?: number,
    message?: string,
    headers?: http.OutgoingHttpHeaders,
  ): void;
}

export const verifyClient = (
  info: VerifyClientInfo,
  callback: VerifyClientCallback,
): void => {
  logSocketAttempt(info.req, info.req.headers['device'] || 'unknown');
  if (!process.env.JWT_KEY) throw new Error('Missing process.env.JWT_KEY');

  const token = info.req.headers.token || '';
  if (!token) {
    logMissing('JWT token');
    callback(false, 401, 'Unauthorized');
  } else {
    jwt.verify(token as string, process.env.JWT_KEY, (err, decoded) => {
      if (!decoded) {
        logMissing('decoded username at JWT Token');
        callback(false, 400, 'Missing decoded username');
        return;
      }
      if (err) {
        callback(false, 401, 'Unauthorized');
        logInvalid('token');
      } else {
        info.req.headers.device = (decoded as { device: string }).device;
        callback(true);
      }
    });
  }
};
