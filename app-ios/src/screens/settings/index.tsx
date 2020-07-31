import React from 'react';
import {Text, Layout} from '@ui-kitten/components';
import styleSheet from '../../styles';

export default function Settings() {
  return (
    <Layout style={styleSheet.basicLayout}>
      <Text category="h1">Settings</Text>
    </Layout>
  );
}
