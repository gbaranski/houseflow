import React, { useState, useEffect, useContext } from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Redirect,
  useHistory,
} from 'react-router-dom';
import routes from './routes';
import LoginPage from './pages/login';
import './services/firebase';
import LeftNavigationBar from './components/leftNavigationBar';
import { makeStyles } from '@material-ui/core';
import { UserProvider, UserContext } from './providers/userProvider';
import {
  firebaseAuth,
  convertToFirebaseUser,
  getIdToken,
  getAllowedDevices,
} from './services/firebase';
import LoadingPage from './pages/loading';
import { ToastContainer, toast } from 'react-toastify';
import {
  beginWebsocketConnection,
  setupWebsocketHandlers,
  getDevicesStatus,
} from './services/websocket';
import {
  WebsocketContext,
  WebsocketProvider,
} from './providers/websocketProvider';
import {
  DeviceDataProvider,
  DeviceDataContext,
} from './providers/deviceDataProvider';
import { getDeviceStatus } from './requests';

const useStyles = makeStyles(() => ({
  root: {
    display: 'flex',
  },
}));

const App = () => {
  const classes = useStyles();
  const history = useHistory();
  const [open, setOpen] = useState(true);
  const [authStateLoaded, setAuthStateLoaded] = useState(false);
  const [websocketEstablished, setWebsocketEstablished] = useState(false);
  const [devicesLoaded, setDevicesLoaded] = useState(false);

  const { websocket, setWebsocket } = useContext(WebsocketContext);
  const { devices, setDevices } = useContext(DeviceDataContext);

  const { firebaseUser, setFirebaseUser } = React.useContext(UserContext);
  if (!setFirebaseUser)
    throw new Error('Expected setFirebaseUser to be true when not initalized');
  if (!setDevices)
    throw new Error('Expected setDevices to be true when not initialized');
  if (!setWebsocket)
    throw new Error('Expected setDevices to be true when not initialized');

  useEffect(() => {
    firebaseAuth.onAuthStateChanged(async () => {
      console.log('Auth state changed');
      if (firebaseAuth.currentUser) {
        if (!firebaseUser) {
          const firebaseUser = await convertToFirebaseUser(
            firebaseAuth.currentUser,
          );
          setFirebaseUser(firebaseUser);
          setAuthStateLoaded(true);
        }

        if (history.location.pathname === '/login') {
          history.replace('/welcome');
        }
      } else {
        setAuthStateLoaded(true);
      }
    });
  }, []);

  useEffect(() => {
    if (websocket) return;
    if (!firebaseUser) return;
    const establishWebsocket = async () => {
      const allowedDevices = getAllowedDevices(firebaseUser);
      const newWebsocket = await beginWebsocketConnection(await getIdToken());

      newWebsocket.onerror = (error) => console.error(error);
      newWebsocket.onclose = (event) =>
        console.log(`Closed connection | Reason: ${event.reason}`);
      newWebsocket.onopen = async () => {
        console.log('Connection open!');
        setWebsocket(newWebsocket);
        setupWebsocketHandlers(newWebsocket, setDevices, () => {});
        getDevicesStatus(newWebsocket, await allowedDevices, () =>
          setDevicesLoaded(true),
        );
      };
    };
    establishWebsocket();
  }, [firebaseUser]);

  if (!authStateLoaded) {
    return <LoadingPage title="Retreiving user data" />;
  }
  if (!websocket || !websocket.OPEN) {
    return <LoadingPage title="Establishing WebSocket connection" />;
  }
  if (!devicesLoaded) {
    return <LoadingPage title="Loading user devices" />;
  }

  const SafeRoute = ({ children, ...rest }: any) => {
    if (rest.protected) {
      return (
        <Route
          {...rest}
          render={({ location }) =>
            firebaseUser ? (
              children
            ) : (
              <Redirect
                to={{
                  pathname: '/login',
                  state: { from: location },
                }}
              />
            )
          }
        />
      );
    } else {
      return <Route {...rest} render={({ location }) => children} />;
    }
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };
  const handleDrawerOpen = () => {
    setOpen(true);
  };
  return (
    <div className={classes.root}>
      {firebaseUser && (
        <>
          <LeftNavigationBar
            open={open}
            handleDrawerClose={handleDrawerClose}
            handleDrawerOpen={handleDrawerOpen}
          />
        </>
      )}
      <Switch>
        {routes.map((route, index) => (
          <SafeRoute
            key={index}
            path={route.path}
            exact={route.exact}
            protected={route.protected}
            name={route.name}
            children={<route.main />}
          />
        ))}
        <Route path={'/login'} exact>
          {!firebaseUser && <LoginPage />}
        </Route>
        {/* <Route path={'/login/success'} exact>
          <LoginSuccess />
        </Route> */}
      </Switch>
    </div>
  );
};

ReactDOM.render(
  <Router>
    <UserProvider>
      <WebsocketProvider>
        <DeviceDataProvider>
          <ToastContainer />
          <App />
        </DeviceDataProvider>
      </WebsocketProvider>
    </UserProvider>
  </Router>,

  document.getElementById('root'),
);
