import fs from 'fs-extra';
import md5File from 'md5-file';

const binPath = `${process.env.PWD}/bin`

export async function findBinaryFile(macAddr: string): Promise<{ path: string; md5: string; }> {
    const binFilePath = `${binPath}/${macAddr}.bin`;

    const binary = await fs.pathExists(binFilePath);
    if (!binary) throw new Error(`Binary not found ${binFilePath}`);

    const md5 = md5File(binFilePath);

    return {
        md5: await md5,
        path: binFilePath
    };
}