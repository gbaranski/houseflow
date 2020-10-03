import React from 'react';
import { GithubOutlined } from '@ant-design/icons';
import { DefaultFooter } from '@ant-design/pro-layout';

export default () => (
  <DefaultFooter
    copyright={new Date().getFullYear().toString()}
    links={[
      {
        key: 'website',
        title: 'Control Home',
        href: 'https://github.com/gbaranski/Control-Home',
        blankTarget: true,
      },
      {
        key: 'github',
        title: <GithubOutlined />,
        href: 'https://github.com/gbaranski/Control-Home',
        blankTarget: true,
      },
    ]}
  />
);
