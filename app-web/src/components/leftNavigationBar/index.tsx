import React from 'react';
import clsx from 'clsx';
import Drawer from '@material-ui/core/Drawer';
import { makeStyles, IconButton, Divider, List } from '@material-ui/core';
import ChevronLeftIcon from '@material-ui/icons/ChevronLeft';
import Appbar from '../appbar';
import { Device, AnyDeviceData } from '@gbaranski/types';
import { NavigationList } from './NavigationList';

const drawerWidth = 240;
const useStyles = makeStyles((theme) => ({
  drawerPaper: {
    position: 'relative',
    whiteSpace: 'nowrap',
    width: drawerWidth,
    transition: theme.transitions.create('width', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.enteringScreen,
    }),
  },
  drawerPaperClose: {
    overflowX: 'hidden',
    transition: theme.transitions.create('width', {
      easing: theme.transitions.easing.sharp,
      duration: theme.transitions.duration.leavingScreen,
    }),
    width: theme.spacing(7),
    [theme.breakpoints.up('sm')]: {
      width: theme.spacing(9),
    },
  },
  toolbarIcon: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'flex-end',
    padding: '0 8px',
    ...theme.mixins.toolbar,
  },
}));

interface props {
  open: boolean;
  handleDrawerOpen: () => any;
  handleDrawerClose: () => any;
  devices: Device.ActiveDevice<AnyDeviceData>[];
}

function LeftNavigationBar(props: props) {
  const classes = useStyles();
  return (
    <>
      <Appbar
        open={props.open}
        handleDrawerOpen={props.handleDrawerOpen}
        devices={props.devices}
      />
      <Drawer
        variant="permanent"
        classes={{
          paper: clsx(
            classes.drawerPaper,
            !props.open && classes.drawerPaperClose,
          ),
        }}
        open={props.open}
      >
        <div className={classes.toolbarIcon}>
          <IconButton onClick={props.handleDrawerClose}>
            <ChevronLeftIcon />
          </IconButton>
        </div>
        <Divider />
        <List>{NavigationList(props.devices)}</List>
      </Drawer>
    </>
  );
}

export default LeftNavigationBar;
