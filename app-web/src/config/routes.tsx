import React from 'react';
import Dashboard from '../pages/dashboard';
import { Redirect } from 'react-router-dom';
import Welcome from '../pages/welcome';
import PanToolIcon from '@material-ui/icons/PanTool';
import DashboardIcon from '@material-ui/icons/Dashboard';

interface Route {
  path: string;
  name: string;
  main: () => JSX.Element;
  navIcon?: () => JSX.Element;
  exact: boolean;
  protected: boolean;
  showOnNavbar: boolean;
}

const routes: Route[] = [
  {
    path: '/',
    name: 'root',
    main: () => <Redirect to={{ pathname: '/welcome' }} />,
    exact: true,
    protected: true,
    showOnNavbar: false,
  },
  {
    path: '/welcome',
    name: 'Welcome',
    main: () => <Welcome />,
    exact: true,
    protected: true,
    showOnNavbar: true,
    navIcon: () => <PanToolIcon />,
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    main: () => <Dashboard />,
    exact: true,
    protected: true,
    showOnNavbar: true,
    navIcon: () => <DashboardIcon />,
  },
];

export default routes;
