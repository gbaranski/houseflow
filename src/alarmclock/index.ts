import fetch, { Headers } from 'node-fetch';
import { AlarmRequestType } from '../types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';

export default class Alarmclock {
  private alarmClockData: any;

  private isProcessing: boolean = false;

  private url: string = 'http://192.168.1.110';

  async handleRequest(req: any, res: any, requestType: AlarmRequestType) {
    if (!isAuthenticated(req.header('username'), req.header('password'))) {
      console.log(`${req.ip} with ${req.hostname} on ${requestType} not authenticated`);
      res.status(401).end();
      return;
    }
    console.log(`${req.ip} with ${req.hostname} on ${requestType} authenticated`);
    const headers = new Headers();
    switch (requestType) {
      case AlarmRequestType.GET_DATA:
        res.json(JSON.stringify(this.alarmClockData));
        break;
      case AlarmRequestType.TEST_ALARM:
        await res.status(await this.fetchUrl(AlarmRequestType.TEST_ALARM, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username'), `alarmclock${requestType}`);
        }
        break;
      case AlarmRequestType.SET_TIME:
        headers.append('time', req.header('time'));
        await res.status(await this.fetchUrl(AlarmRequestType.SET_TIME, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username'), `alarmclock${requestType}`);
        }
        break;
      case AlarmRequestType.SWITCH_STATE:
        headers.append('state', req.header('state'));
        await res.status(await this.fetchUrl(AlarmRequestType.SWITCH_STATE, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username'), `alarmclock${requestType}`);
        }
        break;
      default:
        res.status(500).end();
        break;
    }
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
      .then(response => {
        this.alarmClockData = response.json();
        console.log(`Fetched alarmclock data with response ${response.status}`);
      })
      .finally(() => {
        this.isProcessing = false;
      });
  }
}
