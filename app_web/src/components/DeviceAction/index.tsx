import React, { useState } from 'react';
import { message, Popconfirm, Tooltip } from 'antd';
import Icon from '@mdi/react';

interface DeviceAction {
  mdiIconPath: string;
  toolTipTitle: string;
  onSubmit: () => Promise<any>;
}

const DeviceAction: React.FC<DeviceAction> = ({ mdiIconPath, toolTipTitle, onSubmit }) => {
  const [tooltipVisible, setTooltipVisible] = useState(false);
  const [popconfirmVisible, setPopconfirmVisible] = useState(false);
  const [loading, setLoading] = useState(false);

  return (
    <Tooltip
      title={toolTipTitle}
      visible={tooltipVisible}
      onVisibleChange={setTooltipVisible}
      placement="bottom"
    >
      <Popconfirm
        title={`Are you sure to ${toolTipTitle.toLowerCase()} now?`}
        visible={popconfirmVisible}
        okButtonProps={{ loading }}
        onCancel={() => {
          setPopconfirmVisible(false);
        }}
        onConfirm={() => {
          setLoading(true);
          onSubmit()
            .then(() => {
              message.success('Success!');
            })
            .finally(() => {
              setLoading(false);
              setPopconfirmVisible(false);
            })
            .catch((e) => {
              message.error(e.message);
            });
        }}
      >
        <a onClick={() => setPopconfirmVisible(true)}>
          <Icon path={mdiIconPath} size={1} />
        </a>
      </Popconfirm>
    </Tooltip>
  );
};
export default DeviceAction;
