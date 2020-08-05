import React from 'react';
import Dashboard from './pages/dashboard';
import Alarmclock from './pages/alarmclock';
import Watermixer from './pages/watermixer';
import { Redirect } from 'react-router-dom';
import Welcome from './pages/welcome';

const routes = [
  {
    path: '/',
    name: 'root',
    main: () => <Redirect to={{ pathname: '/welcome' }} />,
    exact: true,
    protected: true,
  },
  {
    path: '/welcome',
    name: 'Welcome',
    main: () => <Welcome />,
    exact: true,
    protected: true,
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    main: () => <Dashboard />,
    exact: true,
    protected: true,
  },
  {
    path: '/alarmclock',
    name: 'Alarmclock',
    main: () => <Alarmclock />,
    exact: true,
    protected: true,
  },
  {
    path: '/watermixer',
    name: 'Watermixer',
    main: () => <Watermixer />,
    exact: true,
    protected: true,
  },
];

export default routes;
