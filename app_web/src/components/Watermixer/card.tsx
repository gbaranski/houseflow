import React from 'react';
import { Card, Statistic, Row, Col } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import { TimestampFunc } from '@/models/relay';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import PageLoading from '@/components/PageLoading';
import { mdiShowerHead } from '@mdi/js';
import DeviceAction from '@/components/DeviceAction';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

const mixingTimestampFunc: TimestampFunc = () =>
  Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;

interface WatermixerCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const WatermixerCard: React.FC<WatermixerCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { sendRelaySignal } = useModel('relay');

  const hasElapsed = (timestamp: number) => Date.now() > timestamp;
  // TODO: fix later
  if (!mqtt) return <PageLoading />;

  const startMixing = () => sendRelaySignal(device, mqtt, mixingTimestampFunc);

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
