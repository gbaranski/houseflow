import * as React from 'react';
import styleSheet from '../styles';
import MaterialIcons from 'react-native-vector-icons/MaterialCommunityIcons';
import {Icon} from '@ui-kitten/components';
export const switchIconOn = () => (
  <MaterialIcons
    name="toggle-switch"
    color={'#32a852'}
    size={28}
    style={styleSheet.switchIcon}
  />
);
export const switchIconOff = () => (
  <MaterialIcons
    name="toggle-switch-off"
    color={'#ff453a'}
    size={28}
    style={styleSheet.switchIcon}
  />
);
export const refreshIcon = (props: any) => (
  <Icon {...props} name={'refresh-outline'} />
);
export const addAlarmIcon = () => (
  <MaterialIcons name="alarm-plus" size={18} color={'#ffff'} />
);
export const testAlarmIcon = () => (
  <MaterialIcons name="do-not-disturb" size={18} color={'#ffff'} />
);

export const WaterIcon = (props: any) => (
  <Icon {...props} name="droplet-outline" />
);
export const WaterIconFill = (props: any) => <Icon {...props} name="droplet" />;

export const AlarmIcon = (props: any) => (
  <Icon {...props} name="clock-outline" />
);
export const AlarmIconFill = (props: any) => <Icon {...props} name="clock" />;

export const SettingsIcon = (props: any) => (
  <Icon {...props} name="settings-outline" />
);
export const SettingsIconFill = (props: any) => (
  <Icon {...props} name="settings" />
);

export const AlertIcon = (props: any) => (
  <Icon {...props} name="alert-circle-outline" />
);
