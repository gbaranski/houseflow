import fetch from 'node-fetch';

import { isAuthenticated } from '../auth';

export default class Alarmclock {
  private alarmClockData: any;

  handleRequest(req: any, res: any) {
    if (isAuthenticated(req.body.authKey)) {
      res.send(this.alarmClockData);
      this.alarmClockData = 0;
    } else {
      res.json(`Not authenticated with ${req.body.authKey}`);
    }
  }

  async fetchEspDataInterval() {
    this.alarmClockData = await (await fetch('http://192.168.1.110/getESPData')).json();
    console.log(this.alarmClockData);
  }
}
