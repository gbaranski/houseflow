import { Device } from '@gbaranski/types';
import { message } from 'antd';

export default () => {
  const mixWater = (uid: string, socket: SocketIOClient.Socket) => {
    console.log('Mixing water');
    const req: Device.RequestDevice = {
      topic: {
        name: 'startmix',
        uid,
      },
    };
    socket.emit('device_request', JSON.stringify(req), (data: string) => {
      message.info(data);
    });
  };

  return {
    mixWater,
  };
};
