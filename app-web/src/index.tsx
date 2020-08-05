import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Redirect,
} from 'react-router-dom';
import routes from './routes';
import LoginPage from './pages/login';
import './services/firebase';
import LeftNavigationBar from './components/leftNavigationBar';
import { makeStyles } from '@material-ui/core';
import { UserProvider, UserContext } from './providers/userProvider';

const useStyles = makeStyles(() => ({
  root: {
    display: 'flex',
  },
}));

const App = () => {
  const classes = useStyles();
  const { firebaseUser } = React.useContext(UserContext);
  const [open, setOpen] = useState(true);

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
      <App />
    </UserProvider>
  </Router>,

  document.getElementById('root'),
);
