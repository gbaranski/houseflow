import { IRoute } from 'umi';

export const routes: IRoute[] = [
  {
    path: '/user',
    layout: false,
    routes: [
      {
        name: 'login',
        path: '/user/login',
        component: './user/login',
      },
    ],
  },
  {
    path: '/welcome',
    name: 'welcome',
    icon: 'smile',
    component: './Welcome',
  },
  {
    path: '/devices',
    name: 'Devices',
    icon: 'Wifi',
    component: './devices/',
  },
  {
    path: '/admin',
    name: 'admin',
    icon: 'crown',
    access: 'canAdmin',
    routes: [
      {
        name: 'Manager',
        path: '/admin/manager',
        component: './admin/manager',
      },
      {
        name: 'Statistics',
        path: '/admin/statistics',
        component: './admin/statistics',
      },
    ],
  },
  {
    path: '/',
    redirect: '/welcome',
  },
  {
    component: './404',
  },
];
