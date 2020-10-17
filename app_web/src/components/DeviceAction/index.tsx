import React, { useState } from 'react';
import { Popconfirm, Tooltip } from 'antd';
import Icon from '@mdi/react';

interface DeviceAction {
  mdiIconPath: string;
  toolTipTitle: string;
  onSubmit: () => any;
}

const DeviceAction: React.FC<DeviceAction> = ({ mdiIconPath, toolTipTitle, onSubmit }) => {
  const [tooltipVisible, setTooltipVisible] = useState(false);

  const onPopconfirmVisiblityChange = (visiblity: boolean) => {
    if (visiblity) setTooltipVisible(false);
  };

  return (
    <Tooltip
      title={toolTipTitle}
      visible={tooltipVisible}
      onVisibleChange={setTooltipVisible}
      placement="bottom"
    >
      <Popconfirm
        title={`Are you sure to ${toolTipTitle.toLowerCase()} now?`}
        onVisibleChange={onPopconfirmVisiblityChange}
        onConfirm={onSubmit}
      >
        <a>
          <Icon path={mdiIconPath} size={1} />
        </a>
      </Popconfirm>
    </Tooltip>
  );
};
export default DeviceAction;
