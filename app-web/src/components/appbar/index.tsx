import React, { useContext } from 'react';
import clsx from 'clsx';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import IconButton from '@material-ui/core/IconButton';
import Typography from '@material-ui/core/Typography';
import MenuIcon from '@material-ui/icons/Menu';
import { makeStyles } from '@material-ui/core';
import { useLocation } from 'react-router-dom';
import { capitalizeFirst } from '../../utils';
import { Device, AnyDeviceData } from '@gbaranski/types';
import { DeviceDataContext } from '../../providers/deviceDataProvider';

const useStyles = makeStyles((theme) => ({
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
  },
  toolbar: {
    paddingRight: 24, // keep right padding when drawer closed
  },
  title: {
    flexGrow: 1,
  },
  appBarShift: {
    marginLeft: 240,
    width: `calc(100% - ${240}px)`,
    transition: theme.transitions.create(['width', 'margin'], {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  },
  menuButton: {
    marginRight: 36,
  },
  menuButtonHidden: {
    display: 'none',
  },
}));

const getCurrentName = (path: string, devices: Device.FirebaseDevice[]) => {
  if (!path.startsWith('/device/')) return capitalizeFirst(path.slice(1));
  const deviceUid = path.replace('/device/', '');
  const activeDevice = devices.find((device) => device.uid === deviceUid);
  if (!activeDevice) throw new Error('Couldnt find device');

  return capitalizeFirst(activeDevice.type.toLowerCase());
};

export default function Appbar(props: {
  open: boolean;
  handleDrawerOpen: any;
}) {
  const classes = useStyles();

  const { firebaseDevices } = useContext(DeviceDataContext);
  return (
    <AppBar
      position="absolute"
      className={clsx(classes.appBar, props.open && classes.appBarShift)}
    >
      <Toolbar className={classes.toolbar}>
        <IconButton
          edge="start"
          color="inherit"
          aria-label="open drawer"
          onClick={props.handleDrawerOpen}
          className={clsx(
            classes.menuButton,
            props.open && classes.menuButtonHidden,
          )}
        >
          <MenuIcon />
        </IconButton>
        <Typography
          component="h1"
          variant="h6"
          color="inherit"
          noWrap
          className={classes.title}
        >
          {getCurrentName(useLocation().pathname, firebaseDevices)}
        </Typography>
      </Toolbar>
    </AppBar>
  );
}
