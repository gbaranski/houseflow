import React from 'react';
import { ThemeProvider } from '@material-ui/core';
import { BrowserRouter, Switch, Route, Link, Redirect } from 'react-router-dom';
import { createMuiTheme } from '@material-ui/core/styles';
import { SignIn } from './pages/SignIn';
import SignUp from './pages/SignUp';

const theme = createMuiTheme({
  palette: {
    type: 'dark',
  },
});

function App() {
  return (
    <BrowserRouter>
      <ThemeProvider theme={theme}>
        <Switch>
          <Route path="/signin">
            <SignIn />
          </Route>
          <Route path="/signup">
            <SignUp />
          </Route>
          <Route
            path="/"
            component={() => {
              window.location.href = 'https://github.com/gbaranski/houseflow';
              return null;
            }}
          />
        </Switch>
      </ThemeProvider>
    </BrowserRouter>
  );
}

export default App;
