import fetch from 'node-fetch';
import { WaterRequestType } from '../types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';

export default class Watermixer {
  private waterMixerData: any;

  private isProcessing: boolean = false;

  private url: string = 'http://192.168.1.120';

  async handleRequest(req: any, res: any, requestType: WaterRequestType) {
    if (!isAuthenticated(req.header('username'), req.header('authKey'))) {
      res.status(401).end();
      return;
    }
    switch (requestType) {
      case WaterRequestType.GET_DATA:
        res.json(JSON.stringify(this.waterMixerData));
        break;
      case WaterRequestType.START_MIXING:
        await res.status(await this.fetchUrl(WaterRequestType.START_MIXING)).end();
        break;
      default:
        res.status(500).end();
        break;
    }
    sendMessage(req.header('username'), requestType);
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
      .then(res => res.json())
      .then(data => {
        this.waterMixerData = data;
        console.log(data);
      })
      .finally(() => {
        this.isProcessing = false;
      });
  }
}
