import fetch from 'node-fetch';

import { isAuthenticated } from '../auth';

export default class Alarmclock {
  private alarmClockData: any;

  handleRequest(req: any, res: any) {
    if (isAuthenticated(req.body.authKey)) {
      console.log(this.alarmClockData);
      res.json(JSON.stringify(this.alarmClockData));
    } else {
      res.send(`Not authenticated with ${req.body.authKey}`);
    }
  }

  async fetchEspDataInterval() {
    fetch('http://192.168.1.110/getESPData')
      .then(data => data.json())
      .then(json => {
        console.log(json);
        this.alarmClockData = json;
      });
  }
}
