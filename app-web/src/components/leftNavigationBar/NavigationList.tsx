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
import { DeviceDataContext } from '../../providers/deviceDataProvider';

const DeviceIcons = (props: { deviceType: Device.DeviceType }) => {
  switch (props.deviceType) {
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
    activeDevices: Device.ActiveDevice<AnyDeviceData>[];
    firebaseDevice: Device.FirebaseDevice;
    key: number;
    className: string | undefined;
    navigate: any;
  }) => (
    <ListItem
      button
      key={props.key}
      disabled={
        !props.activeDevices.some(
          (activeDevice) => activeDevice.uid === props.firebaseDevice.uid,
        )
      }
      onClick={props.navigate}
      selected={props.className !== undefined}
    >
      <ListItemIcon>
        <DeviceIcons deviceType={props.firebaseDevice.type} />
      </ListItemIcon>
      <ListItemText primary={props.firebaseDevice.type} />
    </ListItem>
  ),
);

export const NavigationList = (props: {}) => {
  const { firebaseDevices, activeDevices } = React.useContext(
    DeviceDataContext,
  );

  return (
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
      {firebaseDevices.map((device, index) => (
        <NavLink
          to={`/device/${device.uid}`}
          activeClassName="selected"
          key={index}
          // @ts-ignore
          firebaseDevice={device}
          activeDevices={activeDevices}
          component={FancyDeviceLink}
        />
      ))}
    </div>
  );
};
