import { SpinLoading } from 'antd-mobile';
import styles from './index.module.less';

export const PageLoading = () => (
  <div className={styles.PageLoading}>
    <SpinLoading />
  </div>
);
