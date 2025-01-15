import { Suspense } from 'react';
import { Outlet } from 'react-router-dom';
import Header from './header';
import Footer from './footer';
import styles from './index.module.less';

import { PageLoading } from '../components/page-loading';

const Layout = () => {
  return (
    <Suspense fallback={<PageLoading />}>
      <div className={styles.app}>
        <Header />

        <div className={styles.content}>
          <Outlet />
        </div>

        <Footer />
      </div>
    </Suspense>
  );
};

export default Layout;
