import express, { Request, Response, NextFunction } from 'express';
import esp8266Router from './esp8266'

const router = express.Router();

router.use((err: Error, req: Request, res: Response, next: NextFunction) => {
    console.log(`${err.message} IP: ${req.headers.host}`);
    res.sendStatus(401);
})

router.use('/esp8266', esp8266Router);

export default router;
