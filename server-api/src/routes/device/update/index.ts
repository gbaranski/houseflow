import express, { Request, Response, NextFunction } from 'express';
import fs, { readSync } from 'fs'
import { findBinaryFile } from './misc';

const router = express.Router();

router.use((err: Error, req: Request, res: Response, next: NextFunction) => {
    console.log(`${err.message} IP: ${req.headers.host}`);
    res.sendStatus(401);
})

router.use('/esp8266', async (req, res): Promise<any> => {

    const staMac = req.get('x-esp8266-sta-mac');
    const sdkVersion = req.get("x-esp8266-sdk-version");
    const sketchMd5 = req.get('x-esp8266-sketch-md5');
    if (!sdkVersion) throw new Error("SDK Version not defined");
    if (!staMac) throw new Error("STA Mac not defined");
    if (!sketchMd5) throw new Error("Sketch MD5 not defined");

    try {
        const file = await findBinaryFile(staMac);
        console.log({ devMd5: sketchMd5, fMd5: file.md5 });
        if (file.md5 === sketchMd5) return res.sendStatus(304);

        res.download(file.path, (err) => err !== undefined ?? console.log(err));
    } catch (e) {
        console.log(e.message);
        res.sendStatus(500);
    }
});

export default router;
