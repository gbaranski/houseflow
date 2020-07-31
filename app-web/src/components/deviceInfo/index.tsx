import React from 'react';
import ListItem from '@material-ui/core/ListItem';
import ListItemAvatar from '@material-ui/core/ListItemAvatar';
import ListItemText from '@material-ui/core/ListItemText';
import Title from '../title';

interface deviceInfoElement {
  title: string;
  description: string;
  icon?: any;
}

function ParsedDeviceInfo(props: any) {
  return props.data.map((_data: deviceInfoElement, index: number) => (
    <ListItem key={index}>
      {_data.icon && <ListItemAvatar>{_data.icon}</ListItemAvatar>}
      <ListItemText primary={_data.title} secondary={_data.description} />
    </ListItem>
  ));
}

function DeviceInfo(props: any) {
  return (
    <React.Fragment>
      <Title>Device Info</Title>
      <ParsedDeviceInfo data={props.data} />
    </React.Fragment>
  );
}

export default DeviceInfo;
