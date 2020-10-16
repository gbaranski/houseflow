import React from 'react';
import { GithubOutlined } from '@ant-design/icons';
import { DefaultFooter } from '@ant-design/pro-layout';

export default () => (
  <DefaultFooter
    copyright={new Date().getFullYear().toString()}
    links={[
      {
        key: 'website',
        title: 'Houseflow',
        href: 'https://github.com/gbaranski/houseflow',
        blankTarget: true,
      },
      {
        key: 'github',
        title: <GithubOutlined />,
        href: 'https://github.com/gbaranski/houseflow',
        blankTarget: true,
      },
    ]}
  />
);
