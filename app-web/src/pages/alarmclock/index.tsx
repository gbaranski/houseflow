import React from 'react';
import clsx from 'clsx';
import { makeStyles } from '@material-ui/core/styles';
import CssBaseline from '@material-ui/core/CssBaseline';
import Box from '@material-ui/core/Box';
import Container from '@material-ui/core/Container';
import Grid from '@material-ui/core/Grid';
import Paper from '@material-ui/core/Paper';
import Copyright from '../../components/copyright';
import { MuiPickersUtilsProvider } from '@material-ui/pickers';
import DateFnsUtils from '@date-io/date-fns';

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

function Alarmclock() {
  const classes = useStyles();

  const fixedHeightPaper = clsx(classes.paper, classes.fixedHeight);

  //   const deviceInfo = [
  //     {
  //       title: 'Current state',
  //       description: `Water should be ${data?.isTimerOn ? 'warm' : 'cold'}`,
  //       icon: <Icon path={mdiWater} size={2} color="rgb(117,117,117)" />,
  //     },
  //     {
  //       title: 'Remaining time',
  //       description: data ? parseSeconds(data.remainingSeconds) : '0',
  //       icon: (
  //         <Icon
  //           path={mdiClock}
  //           size={1.5}
  //           color="rgb(117,117,117)"
  //           style={{ marginLeft: '8%' }}
  //         />
  //       ),
  //     },
  //   ];

  return (
    <MuiPickersUtilsProvider utils={DateFnsUtils}>
      <CssBaseline />
      <main className={classes.content}>
        <div className={classes.appBarSpacer} />
        <Container maxWidth="lg" className={classes.container}>
          <Grid container spacing={3}>
            <Grid item xs={12} md={8} lg={12}>
              <Paper className={fixedHeightPaper}>
                {/* <DeviceInfo data={deviceInfo} /> */}
              </Paper>
            </Grid>
            <Grid item xs={12}>
              <Paper className={classes.paper}>
                {/* <DeviceManager
                  data={[
                    {
                      onClick: handleStartMixing,
                      innerText: 'Start mixing',
                    },
                  ]}
                /> */}
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

export default Alarmclock;
