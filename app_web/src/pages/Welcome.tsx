import React from 'react';
import { PageContainer } from '@ant-design/pro-layout';
import { Card, Typography } from 'antd';

export default (): React.ReactNode => (
  <PageContainer>
    <Card>
      <Typography.Title level={4}>
        <span role="img" aria-label="icon">
          🚧
        </span>
        Houseflow - Alpha
        <span role="img" aria-label="icon">
          {' '}
          🚧
        </span>
      </Typography.Title>
      <Typography>
        <Typography.Paragraph>
          This Web App is currently under heavy development 🏗 . You might find a lot of bugs or even
          dragons{' '}
          <span role="img" aria-label="icon">
            {' '}
            🐉
          </span>
          ...
          <Typography.Text strong>
            Please use this app on your own risk{' '}
            <span role="img" aria-label="icon">
              👷
            </span>
            .
          </Typography.Text>
        </Typography.Paragraph>
      </Typography>
    </Card>
  </PageContainer>
);
