import fetch from 'node-fetch';
import { WaterRequestType } from '../types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';

export default class Watermixer {
  private waterMixerData: any;

  private isProcessing: boolean = false;

  private url: string = 'http://192.168.1.120';

  async handleRequest(req: any, res: any, requestType: WaterRequestType) {
    if (!isAuthenticated(req.header('username'), req.header('password'))) {
      console.log(`${req.ip} with ${req.hostname} on ${requestType} not authenticated`);
      res.status(401).end();
      return;
    }
    console.log(`${req.ip} with ${req.hostname} on ${requestType} authenticated`);
    switch (requestType) {
      case WaterRequestType.GET_DATA:
        res.json(JSON.stringify(this.waterMixerData));
        break;
      case WaterRequestType.START_MIXING:
        await res.status(await this.fetchUrl(WaterRequestType.START_MIXING)).end();
        if (req.header('username') !== 'gbaranski') {
          sendMessage(req.header('username'), `watermixer${requestType}`);
        }
        break;
      default:
        res.status(500).end();
        break;
    }
  }

  async fetchUrl(path: string): Promise<number> {
    this.isProcessing = true;
    let statusCode = 0;
    await fetch(this.url + path, {
      method: 'POST',
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
    fetch(this.url + WaterRequestType.GET_DATA)
      .then(response => {
        this.waterMixerData = response.json();
        console.log(`Fetched watermixer data with response ${response.status}`);
      })
      .finally(() => {
        this.isProcessing = false;
      });
  }
}
