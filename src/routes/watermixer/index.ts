import express from 'express';
import { WaterRequestType, RequestHistory } from '@gbaranski/types';
import { fetchURL, getIpStr } from '../../helpers';
import { WATERMIXER_URL } from '../../config';
import { sendMessage } from '../../firebase';
import { createHistory, setProcessing, getProcessing } from '../globals';
import { getData } from './interval';

const router = express.Router();

export const setProcessingWatermixer = (state: boolean): void => {
  setProcessing({
    ...getProcessing(),
    watermixer: state,
  });
};

router.post(
  '/startMixing',
  async (req, res): Promise<void> => {
    const reqHistory: RequestHistory = {
      user: req.header('username') || '',
      requestType: WaterRequestType.START_MIXING,
      date: new Date(),
      ip: getIpStr(req),
    };
    createHistory(reqHistory);

    setProcessingWatermixer(true);
    res
      .status(await fetchURL(WATERMIXER_URL, WaterRequestType.START_MIXING))
      .end();
    setProcessingWatermixer(false);

    sendMessage(
      req.header('username') || '',
      `watermixer${WaterRequestType.START_MIXING}`,
    );
  },
);

router.get('/getData', (req, res): void => {
  res.json(JSON.stringify(getData()));
});

export default router;
