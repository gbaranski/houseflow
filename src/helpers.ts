import express from 'express';
import fs from 'fs-extra';
import { RequestHistory } from './types';

const historyFile = './temp/req_history.json';

export function getIp(req: express.Request) {
  return (
    req.headers['cf-connecting-ip'] ||
    req.headers['x-forwarded-for'] ||
    req.connection.remoteAddress
  );
}
export function getIpStr(req: express.Request) {
  return String(getIp(req));
}

async function createFile() {
  await fs.createFile(historyFile);
  await fs.writeJson(historyFile, []);
  return 0;
}

export async function createHistory(data: RequestHistory) {
  try {
    if (!(await fs.pathExists(historyFile))) {
      await createFile();
    }
    const fileData = await fs.readJson(historyFile);
    fileData.push(data);
    await fs.outputJson(historyFile, fileData);
  } catch (e) {
    console.log(e);
  }
}

export function getHistory() {
  return fs.readJsonSync(historyFile);
}
