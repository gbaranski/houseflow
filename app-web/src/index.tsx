import React, {useState, useEffect} from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Redirect,
} from 'react-router-dom';
import routes from './routes';
import LoginPage from './pages/login';
import LoginLoading from './pages/loginLoading';
import {login} from './requests';
import {initializeFirebase} from './services/firebase';
import LeftNavigationBar from './components/leftNavigationBar';
import {makeStyles} from '@material-ui/core';

const useStyles = makeStyles(() => ({
  root: {
    display: 'flex',
  },
}));

const getLoginPage = async (setLoggedIn: any, setAttemptedToLogin: any) => {
  const res = login();
  if (await res) {
    setLoggedIn(true);
  }
  res.then(() => setAttemptedToLogin(true));
};

const App = () => {
  const classes = useStyles();
  useEffect(() => {
    initializeFirebase();
  }, []);

  const [isLoggedIn, setLoggedIn] = useState(false);
  const [isAttemptedToLogin, setAttemptedToLogin] = useState(false);
  const [open, setOpen] = useState(true);

  if (!isAttemptedToLogin) {
    getLoginPage(setLoggedIn, setAttemptedToLogin);
    return <LoginLoading />;
  }
  const SafeRoute = ({children, ...rest}: any) => {
    if (rest.protected) {
      return (
        <Route
          {...rest}
          render={({location}) =>
            isLoggedIn ? (
              children
            ) : (
              <Redirect
                to={{
                  pathname: '/login',
                  state: {from: location},
                }}
              />
            )
          }
        />
      );
    } else {
      return <Route {...rest} render={({location}) => children} />;
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
      {isLoggedIn && (
        <>
          <Redirect
            to={{
              pathname: '/',
            }}
          />
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
          {!isLoggedIn && (
            <LoginPage
              setAttemptedToLogin={setAttemptedToLogin}
              setLoggedIn={setLoggedIn}
            />
          )}
        </Route>
      </Switch>
    </div>
  );
};

ReactDOM.render(
  <Router>
    <App />
  </Router>,

  document.getElementById('root'),
);
