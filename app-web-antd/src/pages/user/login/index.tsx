import React from 'react';
import Footer from '@/components/Footer';
import styles from './style.less';
import LoginEmail from './components/loginEmail';
import LoginGoogle from './components/loginGoogle';

const Login: React.FC<{}> = () => {
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.main}>
          <LoginGoogle />
          <LoginEmail />
        </div>
      </div>
      <Footer />
    </div>
  );
};

export default Login;
