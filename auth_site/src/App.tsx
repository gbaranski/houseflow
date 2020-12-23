import React from 'react';
import { BrowserRouter, Switch, Route } from 'react-router-dom';
import SignIn from './pages/SignIn';
import SignUp from './pages/SignUp';

function App() {
  return (
    <BrowserRouter>
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
    </BrowserRouter>
  );
}

export default App;
