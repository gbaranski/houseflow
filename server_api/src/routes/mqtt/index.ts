import express from 'express';
import mqttAuthRouter from './auth';

const router = express.Router();

router.use('/auth', mqttAuthRouter);
export default router;
