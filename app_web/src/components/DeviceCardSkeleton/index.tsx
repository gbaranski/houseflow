import React, { useState } from 'react';
import { Card, Empty, Modal, Row, Tooltip } from 'antd';
import { capitalizeFirst } from '@/utils/utils';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH, CONTACT_MAIL, GITHUB_URL } from '@/utils/constants';
import Icon from '@mdi/react';
import { mdiHelp } from '@mdi/js';
import { GithubOutlined, MailOutlined } from '@ant-design/icons';

export default (props: { name: string; description: string }) => {
  const [helpModalVisible, setHelpModalVisible] = useState(false);

  return (
    <>
      <Modal
        title="Help"
        visible={helpModalVisible}
        onOk={() => setHelpModalVisible(false)}
        onCancel={() => setHelpModalVisible(false)}
      >
        <p style={{ color: 'rgb(90,90,90)', fontSize: 15 }}>
          {"Looks like you've got a problem, contact us via methods below to get some support "}
        </p>

        <Row justify="space-around">
          <a href={`${GITHUB_URL}/issues`} target="__blank">
            <GithubOutlined style={{ fontSize: 64, color: 'rgb(90,90,90)' }} />
          </a>
          <a href={`mailto:${CONTACT_MAIL}`}>
            <MailOutlined style={{ fontSize: 64, color: 'rgb(90,90,90)' }} />
          </a>
        </Row>
      </Modal>
      <Card
        title={capitalizeFirst(props.name.toLowerCase())}
        style={{ width: CARD_MIN_WIDTH }}
        bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
        actions={[
          <Tooltip title="Help me">
            <a onClick={() => setHelpModalVisible(true)}>
              <Icon path={mdiHelp} size={1} />
            </a>
          </Tooltip>,
        ]}
      >
        <Empty description={props.description} />
      </Card>
    </>
  );
};
