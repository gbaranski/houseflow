import React from 'react';
import { GithubOutlined } from '@ant-design/icons';
import { DefaultFooter } from '@ant-design/pro-layout';
import { GITHUB_URL } from '@/utils/constants';

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
        href: GITHUB_URL,
        blankTarget: true,
      },
    ]}
  />
);
