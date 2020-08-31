import express from 'express';
import updateRouter from './update/index';

const router = express.Router();

router.use('/ota', updateRouter);

export default router;
