import React from 'react';
import { PageContainer } from '@ant-design/pro-layout';
import { Card, Button } from 'antd';

export default (): React.ReactNode => {
  return (
    <PageContainer>
      <Card>
        <Button>Manager</Button>
      </Card>
    </PageContainer>
  );
};
