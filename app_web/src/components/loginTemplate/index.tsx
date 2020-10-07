import React from 'react';
import Footer from '@/components/Footer';
import styles from './style.less';

const LoginRegisterTemplate: React.FC<{}> = (props) => {
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.main}>{props.children}</div>
      </div>
      <Footer />
    </div>
  );
};

export default LoginRegisterTemplate;
