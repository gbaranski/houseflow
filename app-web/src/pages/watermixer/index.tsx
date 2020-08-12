import React from 'react';
import clsx from 'clsx';
import { makeStyles } from '@material-ui/core/styles';
import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';
import Copyright from '../../components/copyright';
import { mdiWater } from '@mdi/js';
import { mdiClock } from '@mdi/js';
import Icon from '@mdi/react';
import DeviceManager from '../../components/deviceManager';
import DeviceInfo from '../../components/deviceInfo';
import DateFnsUtils from '@date-io/date-fns';
import { Watermixer as WatermixerType, Client } from '@gbaranski/types';
import { WebsocketContext } from '../../providers/websocketProvider';
import LoadingPage from '../loading';
import { MuiPickersUtilsProvider } from '@material-ui/pickers';
import { Container, Box, CssBaseline } from '@material-ui/core';
import { preWebsocketMessage } from '../../services/websocket';
import { DeviceDataContext } from '../../providers/deviceDataProvider';
import { useHistory } from 'react-router-dom';
import { parseSeconds } from '../../utils';

const useStyles = makeStyles((theme) => ({
  title: {
    flexGrow: 1,
  },
  appBarSpacer: theme.mixins.toolbar,
  content: {
    flexGrow: 1,
    height: '100vh',
    overflow: 'auto',
  },
  container: {
    paddingTop: theme.spacing(4),
    paddingBottom: theme.spacing(4),
  },
  paper: {
    padding: theme.spacing(2),
    display: 'flex',
    overflow: 'auto',
    flexDirection: 'column',
  },
  fixedHeight: {
    height: 240,
  },
}));

function Watermixer(props: { uid: string }) {
  const classes = useStyles();
  const history = useHistory();
  const { websocket } = React.useContext(WebsocketContext);
  const { activeDevices } = React.useContext(DeviceDataContext);

  if (!websocket || !websocket.OPEN) {
    return <LoadingPage title="Websocket disconnected, reconnecting" />;
  }
  const device = activeDevices.find((_device) => _device.uid === props.uid);
  if (!device) {
    history.replace('/welcome');
    throw new Error('Could not found device');
  }

  const data = device.data as WatermixerType.Data;

  const fixedHeightPaper = clsx(classes.paper, classes.fixedHeight);

  const deviceInfo = [
    {
      title: 'Current state',
      description: `Water should be ${data.isTimerOn ? 'warm' : 'cold'}`,
      icon: <Icon path={mdiWater} size={2} color="rgb(117,117,117)" />,
    },
    {
      title: 'Remaining time',
      description: parseSeconds(data.remainingSeconds),
      icon: (
        <Icon
          path={mdiClock}
          size={1.5}
          color="rgb(117,117,117)"
          style={{ marginLeft: '8%' }}
        />
      ),
    },
  ];

  const handleStartMixing = () => {
    try {
      preWebsocketMessage(websocket);
    } catch (e) {
      console.error(e);
    }
    const request: Client.Request = {
      requestType: 'START_MIXING',
      deviceUid: props.uid,
    };
    websocket.send(JSON.stringify(request));
  };

  return (
    <MuiPickersUtilsProvider utils={DateFnsUtils}>
      <CssBaseline />
      <main className={classes.content}>
        <div className={classes.appBarSpacer} />
        <Container maxWidth="lg" className={classes.container}>
          <Grid container spacing={3}>
            <Grid item xs={12} md={8} lg={12}>
              <Paper className={fixedHeightPaper}>
                <DeviceInfo data={deviceInfo} />
              </Paper>
            </Grid>
            <Grid item xs={12}>
              <Paper className={classes.paper}>
                <DeviceManager
                  data={[
                    {
                      onClick: handleStartMixing,
                      innerText: 'Start mixing',
                    },
                  ]}
                />
              </Paper>
            </Grid>
          </Grid>
          <Box pt={4}>
            <Copyright />
          </Box>
        </Container>
      </main>
    </MuiPickersUtilsProvider>
  );
}

export default Watermixer;
