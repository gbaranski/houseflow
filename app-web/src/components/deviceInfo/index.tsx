import React from 'react';
import ListItem from '@material-ui/core/ListItem';
import ListItemAvatar from '@material-ui/core/ListItemAvatar';
import ListItemText from '@material-ui/core/ListItemText';
import Title from '../title';

export interface DeviceInfoElement {
  title: string;
  description: string;
  icon?: JSX.Element;
}

function DeviceInfo(props: { data: DeviceInfoElement[] }) {
  return (
    <React.Fragment>
      <Title>Device Info</Title>
      {props.data.map((_data: DeviceInfoElement, index: number) => (
        <ListItem key={index}>
          {_data.icon && <ListItemAvatar>{_data.icon}</ListItemAvatar>}
          <ListItemText primary={_data.title} secondary={_data.description} />
        </ListItem>
      ))}
    </React.Fragment>
  );
}

export default DeviceInfo;
