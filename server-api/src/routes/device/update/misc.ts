import fs from 'fs-extra';
import md5File from 'md5-file';
import { Response } from 'express';
import path from 'path';

const binPath = `${process.env.PWD}/bin`;

export interface File {
  path: string;
  md5: string;
  size: number;
}

export async function findBinaryFile(type: string): Promise<File> {
  const binFilePath = `${binPath}/${type.toLowerCase()}.bin`;

  const binary = await fs.pathExists(binFilePath);
  if (!binary) throw new Error(`Binary not found ${binFilePath}`);
  const stats = await fs.stat(binFilePath);

  const size = stats['size'];
  const md5 = md5File(binFilePath);

  return {
    md5: await md5,
    path: binFilePath,
    size,
  };
}

export async function sendBinaryFile(res: Response, file: File) {
  res.set('Content-Type', 'application/octet-stream');
  res.set(
    'Content-Disposition',
    `attachment;filename=${path.basename(file.path)}`,
  );
  res.set('Content-Length', String(file.size));
  res.set('x-MD5', file.md5);
  res.sendFile(file.path);
}
