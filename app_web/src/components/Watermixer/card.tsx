import React from 'react';
import { Card, Statistic, Row, Col } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import { mdiShowerHead } from '@mdi/js';
import DeviceAction from '@/components/DeviceAction';
import { sendDeviceRequest } from '@/services/device';

const hasElapsed = (timestamp: number) => Date.now() > timestamp;

interface WatermixerCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const WatermixerCard: React.FC<WatermixerCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { currentUser } = initialState || {};
  if (!currentUser) throw new Error('Current user is not defined');

  const startMixing = async () =>
    sendDeviceRequest({
      user: {
        token: await currentUser.getIdToken(),
      },
      device: Relay.createRelayRequest({ uid: device.uid }),
    });

  return (
    <Card
      title="Watermixer"
      style={{ minWidth: CARD_MIN_WIDTH }}
      bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
      actions={[
        <DeviceAction
          mdiIconPath={mdiShowerHead}
          toolTipTitle="Mix water"
          onSubmit={startMixing}
        />,
      ]}
    >
      <Row justify="space-around">
        <Col span={10} style={{ textAlign: 'left' }}>
          <Statistic
            title="Mixing state"
            value={hasElapsed(device.data.lastSignalTimestamp) ? 'Idle' : 'Mixing!'}
          />
        </Col>
        <Col span={10} style={{ textAlign: 'right' }}>
          <Statistic.Countdown
            title="Time left"
            value={device.data.lastSignalTimestamp}
            format="mm:ss"
          />
        </Col>
      </Row>
    </Card>
  );
};

export default WatermixerCard;
