import express from 'express';
import fetch, { Headers } from 'node-fetch';
import { AlarmRequestType, AlarmclockData } from '../types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';
import { getIp } from '../helpers';

const SECONDS_IN_DAY = 86400;

export default class Alarmclock {
  private data: any;

  private temperaturesArr: number[] = new Array(SECONDS_IN_DAY);

  private isProcessing: boolean = false;

  private url: string = 'http://192.168.1.110';

  async handleRequest(req: express.Request, res: express.Response, requestType: AlarmRequestType) {
    if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
      console.log(
        `${getIp(req)} with ${req.get('user-agent')} on ${requestType} not authenticated`,
      );
      res.status(401).end();
      return;
    }
    console.log(`${getIp(req)} with ${req.get('user-agent')} on ${requestType} authenticated`);
    const headers = new Headers();
    switch (requestType) {
      case AlarmRequestType.GET_DATA:
        res.json(JSON.stringify(this.data));
        break;
      case AlarmRequestType.GET_TEMP_ARRAY:
        res.json(JSON.stringify(this.temperaturesArr));
        break;
      case AlarmRequestType.TEST_ALARM:
        await res.status(await this.fetchUrl(AlarmRequestType.TEST_ALARM, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username') || '', `alarmclock${requestType}`);
        }
        break;
      case AlarmRequestType.SET_TIME:
        headers.append('time', req.header('time') || '');
        await res.status(await this.fetchUrl(AlarmRequestType.SET_TIME, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username') || '', `alarmclock${requestType}`);
        }
        break;
      case AlarmRequestType.SWITCH_STATE:
        headers.append('state', req.header('state') || '');
        await res.status(await this.fetchUrl(AlarmRequestType.SWITCH_STATE, headers)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username') || '', `alarmclock${requestType}`);
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
      this.temperaturesArr.shift();
      this.temperaturesArr.push(this.data);
      return;
    }
    this.isProcessing = true;
    fetch(this.url + AlarmRequestType.GET_DATA)
      .then(res => res.json())
      .then((data: AlarmclockData) => {
        this.data = data;
        this.temperaturesArr.shift();
        this.temperaturesArr.push(data.temperature);
        console.log('Fetched alarmclock data');
      })
      .catch(error => {
        console.log('Error while fetching alarmclock', error);
      })
      .finally(() => {
        this.isProcessing = false;
      });
  }
}
