import express from 'express';
import { devices } from '@/routes/globals';

const router = express.Router();

export function setWatermixerState(state: boolean): void {
  devices.watermixer.status = state;
}
export function getWatermixerState(): boolean {
  return devices.watermixer.status;
}
router.post(
  '/startMixing',
  async (req, res): Promise<void> => {
    if (!devices.watermixer.ws) {
      res.sendStatus(503);
      return;
    }
    devices.watermixer.ws.send('START_MIXING');
    res.sendStatus(201);
  },
);

router.get('/getData', (req, res): void => {
  res.json(JSON.stringify(devices.watermixer.data));
});

export default router;
