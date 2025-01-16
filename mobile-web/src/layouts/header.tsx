import { NavBar } from 'antd-mobile';
import { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import styles from './header.module.less';

// 头部导航栏
const Header: FC = () => {
  const navigate = useNavigate();

  return (
    <NavBar
      className={styles.top}
      onBack={() => {
        navigate(-1);
      }}
    >
      配合路由使用
    </NavBar>
  );
};

export default Header;
