import React from 'react';
import clsx from 'clsx';
import {makeStyles} from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Box from '@material-ui/core/Box';
import IconButton from '@material-ui/core/IconButton';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';
import TempChart from '../../components/tempChart';
import Copyright from '../../components/copyright';
import {mdiThermometer} from '@mdi/js';
import {mdiWater} from '@mdi/js';
import {mdiClock} from '@mdi/js';
import {mdiAlarm} from '@mdi/js';
import Icon from '@mdi/react';
import DeviceManager from '../../components/deviceManager';
import DeviceInfo from '../../components/deviceInfo';
import Dialog from '@material-ui/core/Dialog';
import DialogTitle from '@material-ui/core/DialogTitle';
import DialogContent from '@material-ui/core/DialogContent';
import DialogContentText from '@material-ui/core/DialogContentText';
import DialogActions from '@material-ui/core/DialogActions';
import CloseIcon from '@material-ui/icons/Close';
import {
  MuiPickersUtilsProvider,
  KeyboardTimePicker,
} from '@material-ui/pickers';
import Button from '@material-ui/core/Button';
import DateFnsUtils from '@date-io/date-fns';
import Snackbar from '@material-ui/core/Snackbar';
import {AlarmclockData} from '@gbaranski/types';
import {useInterval} from '../../helpers';
import {
  getAlarmClockData,
  sendTimeRequest,
  switchAlarmState,
  testSiren,
} from '../../requests';

const useStyles = makeStyles((theme) => ({
  root: {
    display: 'flex',
  },
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

function Alarmclock() {
  const classes = useStyles();

  const dataFromCache = localStorage.getItem('lastAlarmclockData');
  const parsedDataFromCache: AlarmclockData | undefined = dataFromCache
    ? JSON.parse(dataFromCache)
    : undefined;

  const [data, setData] = React.useState<AlarmclockData | undefined>(
    parsedDataFromCache || undefined,
  );
  const [timeDialogOpen, setTimeDialogOpen] = React.useState(false);
  const [selectedDate, setSelectedDate] = React.useState(new Date());
  const [snackbarOpen, setSnackbarOpen] = React.useState(false);
  const [snackbarMessage, setSnackbarMessage] = React.useState('');
  const handleSnackbarClose = () => {
    setSnackbarOpen(false);
  };

  const handleTestAlarm = async () => {
    const res = await testSiren();
    if (res) {
      setSnackbarMessage('Success testing siren!');
    } else {
      setSnackbarMessage(
        "Server is down, or You don't have permisison for that :)!",
      );
    }
    setSnackbarOpen(true);
    setTimeout(() => {
      setSnackbarOpen(false);
    }, 1000);
  };
  const handleSwitchState = async () => {
    console.log('Switching alamr state');
    if (data) {
      await switchAlarmState(data.alarmState);
      setSnackbarMessage('Success changing alarm state!');
      setSnackbarOpen(true);
      setTimeout(() => {
        setSnackbarOpen(false);
      }, 1000);
    } else {
      setSnackbarMessage('Something went wrong!');
      setSnackbarOpen(true);
      setTimeout(() => {
        setSnackbarOpen(false);
      }, 1000);
    }
  };

  const handleSendAlarmTime = async () => {
    setTimeDialogOpen(false);
    const ok = await sendTimeRequest(selectedDate);
    if (ok) {
      setSnackbarMessage('Success changing alarm time!');
    } else {
      setSnackbarMessage('Failed during changing alarm time!');
    }
    setSnackbarOpen(true);
    setTimeout(() => {
      setSnackbarOpen(false);
    }, 1000);
  };

  const handleDateChange = (date: any) => {
    setSelectedDate(date);
  };

  const handleSetAlarmTime = () => {
    setTimeDialogOpen(true);
  };

  useInterval(async () => {
    const alarmclockData = await getAlarmClockData();
    setData(alarmclockData);
    localStorage.setItem('lastAlarmclockData', JSON.stringify(alarmclockData));
  }, 500);

  const fixedHeightPaper = clsx(classes.paper, classes.fixedHeight);

  const deviceInfo = [
    {
      title: 'Temperature',
      description: data?.temperature + '°C / ' + data?.heatIndex + '°C' || '',
      icon: <Icon path={mdiThermometer} size={2} color="rgb(117,117,117)" />,
    },
    {
      title: 'Humidity',
      description: data?.humidity + '%' || '',
      icon: <Icon path={mdiWater} size={2} color="rgb(117,117,117)" />,
    },
    {
      title: 'Remaining time',
      description: data?.remainingTime || '',
      icon: (
        <Icon
          path={mdiClock}
          size={1.5}
          color="rgb(117,117,117)"
          style={{marginLeft: '8%'}}
        />
      ),
    },

    {
      title: 'Alarm time',
      description:
        data?.alarmTime + `${data?.alarmState ? ' ON' : ' OFF'}` || '',
      icon: (
        <Icon
          path={mdiAlarm}
          size={1.5}
          color="rgb(117,117,117)"
          style={{marginLeft: '8%'}}
        />
      ),
    },
  ];

  return (
    <MuiPickersUtilsProvider utils={DateFnsUtils}>
      <div className={classes.root}>
        <Dialog
          open={timeDialogOpen}
          onClose={() => setTimeDialogOpen(false)}
          aria-labelledby="form-dialog-title">
          <DialogTitle id="form-dialog-title">Set new Alarm Time</DialogTitle>
          <DialogContent>
            <DialogContentText>
              Use time input below to set new alarm time, press Save to save
              that to remote ESP microcontroller
            </DialogContentText>
            <KeyboardTimePicker
              margin="normal"
              id="time-picker"
              label="Time picker"
              value={selectedDate}
              required
              onChange={handleDateChange}
              minutesStep={5}
              ampm={false}
              KeyboardButtonProps={{
                'aria-label': 'change time',
              }}
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setTimeDialogOpen(false)} color="primary">
              Cancel
            </Button>
            <Button onClick={() => handleSendAlarmTime()} color="primary">
              Save
            </Button>
          </DialogActions>
        </Dialog>
        <Snackbar
          anchorOrigin={{
            vertical: 'top',
            horizontal: 'right',
          }}
          open={snackbarOpen}
          autoHideDuration={6000}
          onClose={handleSnackbarClose}
          message={snackbarMessage}
          action={
            <React.Fragment>
              <Button
                color="secondary"
                size="small"
                onClick={handleSnackbarClose}>
                UNDO
              </Button>
              <IconButton
                size="small"
                aria-label="close"
                color="inherit"
                onClick={handleSnackbarClose}>
                <CloseIcon fontSize="small" />
              </IconButton>
            </React.Fragment>
          }
        />

        <CssBaseline />

        <main className={classes.content}>
          <div className={classes.appBarSpacer} />
          <Container maxWidth="lg" className={classes.container}>
            <Grid container spacing={3}>
              <Grid item xs={12} md={8} lg={9}>
                <Paper className={fixedHeightPaper}>
                  <TempChart />
                </Paper>
              </Grid>
              <Grid item xs={12} md={4} lg={3}>
                <Paper className={fixedHeightPaper}>
                  <DeviceInfo data={deviceInfo} />
                </Paper>
              </Grid>
              <Grid item xs={12}>
                <Paper className={classes.paper}>
                  <DeviceManager
                    data={[
                      {
                        onClick: handleTestAlarm,
                        innerText: 'Test alarm',
                      },
                      {
                        onClick: handleSetAlarmTime,
                        innerText: 'Set alarm time',
                      },
                      {
                        onClick: handleSwitchState,
                        innerText: 'Switch alarm state',
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
      </div>
    </MuiPickersUtilsProvider>
  );
}

export default Alarmclock;
