import fetch, { Headers } from 'node-fetch';
import { AlarmRequestType } from '../types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';

export default class Alarmclock {
  private alarmClockData: any;

  private isProcessing: boolean = false;

  private url: string = 'http://192.168.1.110';

  async handleRequest(req: any, res: any, requestType: AlarmRequestType) {
    if (!isAuthenticated(req.header('username'), req.header('authKey'))) {
      res.status(401).end();
      return;
    }
    const headers = new Headers();
    switch (requestType) {
      case AlarmRequestType.GET_DATA:
        res.json(JSON.stringify(this.alarmClockData));
        break;
      case AlarmRequestType.TEST_ALARM:
        await res.status(await this.fetchUrl(AlarmRequestType.TEST_ALARM, headers)).end();
        break;
      case AlarmRequestType.SET_TIME:
        headers.append('time', req.header('time'));
        await res.status(await this.fetchUrl(AlarmRequestType.SET_TIME, headers)).end();
        break;
      case AlarmRequestType.SWITCH_STATE:
        headers.append('state', req.header('state'));
        await res.status(await this.fetchUrl(AlarmRequestType.SWITCH_STATE, headers)).end();
        break;
      default:
        res.status(500).end();
        break;
    }
    sendMessage(req.header('username'), requestType);
  }

  async fetchUrl(path: string, headers: Headers): Promise<number> {
    this.isProcessing = true;
    let statusCode = 0;
    await fetch(this.url + path, {
      method: 'POST',
      headers,
    })
      .then(data => {
        console.log('Success:', data.status);
        statusCode = data.status;
      })
      .catch(error => {
        console.error('Error:', error);
        statusCode = 503;
      });
    this.isProcessing = false;
    return statusCode;
  }

  async fetchEspDataInterval() {
    if (this.isProcessing) {
      console.log('Connection overloaded');
      return;
    }
    this.isProcessing = true;
    fetch(this.url + AlarmRequestType.GET_DATA)
      .then(res => res.json())
      .then(data => {
        this.alarmClockData = data;
        console.log(data);
      })
      .finally(() => {
        this.isProcessing = false;
      });
  }
}
