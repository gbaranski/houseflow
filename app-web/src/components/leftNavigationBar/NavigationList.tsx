import React from 'react';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import AlarmIcon from '@material-ui/icons/Alarm';
import OpacityIcon from '@material-ui/icons/Opacity';
import { NavLink } from 'react-router-dom';
import { Device, AnyDeviceData } from '@gbaranski/types';
import ErrorIcon from '@material-ui/icons/Error';
import routes from '../../config/routes';
import { Divider } from '@material-ui/core';

const DeviceIcons = (props: { device: Device.ActiveDevice<AnyDeviceData> }) => {
  switch (props.device.type) {
    case 'ALARMCLOCK':
      return <AlarmIcon />;
    case 'WATERMIXER':
      return <OpacityIcon />;
    default:
      return <ErrorIcon />;
  }
};

const FancyLink = React.forwardRef(
  (props: {
    key: number;
    className: string | undefined;
    navigate: any;
    name: string;
    icon: () => JSX.Element;
  }) => (
    <ListItem
      button
      key={props.key}
      onClick={props.navigate}
      selected={props.className !== undefined}
    >
      <ListItemIcon>
        <props.icon />
      </ListItemIcon>
      <ListItemText primary={props.name} />
    </ListItem>
  ),
);
const FancyDeviceLink = React.forwardRef(
  (props: {
    device: Device.ActiveDevice<AnyDeviceData>;
    key: number;
    className: string | undefined;
    navigate: any;
  }) => (
    <ListItem
      button
      disabled={props.device.status}
      key={props.key}
      onClick={props.navigate}
      selected={props.className !== undefined}
    >
      <ListItemIcon>
        <DeviceIcons device={props.device} />
      </ListItemIcon>
      <ListItemText primary={props.device.type} />
    </ListItem>
  ),
);

export const NavigationList = (
  devices: Device.ActiveDevice<AnyDeviceData>[],
) => (
  <div>
    {routes
      .filter((route) => route.showOnNavbar === true)
      .map((route, index) => (
        <NavLink
          to={route.path}
          activeClassName="selected"
          key={index}
          // @ts-ignore
          name={route.name}
          icon={route.navIcon || ErrorIcon}
          component={FancyLink}
        />
      ))}
    <Divider />
    {devices.map((device, index) => (
      <NavLink
        to={`/device/${device.uid}`}
        activeClassName="selected"
        key={index}
        // @ts-ignore
        device={device}
        component={FancyDeviceLink}
      />
    ))}
  </div>
);
