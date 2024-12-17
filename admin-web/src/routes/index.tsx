import { createBrowserRouter, Navigate, RouterProvider } from 'react-router-dom';
import { lazy } from 'react';

const Layout = lazy(() => import('@/layouts'));
const Home = lazy(() => import('@/pages/home'));
const Todo = lazy(() => import('@/pages/todo'));
const Message = lazy(() => import('@/pages/message'));
const PersonalCenter = lazy(() => import('@/pages/personal-center'));

export interface MetaProps {
  keepAlive?: boolean;
  requiresAuth?: boolean;
  title: string;
  key?: string;
}

export interface RouteObject {
  caseSensitive?: boolean;
  children?: RouteObject[];
  element?: React.ReactNode;
  index?: false;
  path?: string;
  meta?: MetaProps;
  isLink?: string;
}

const rootRouter: RouteObject[] = [
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        path: '',
        element: <Home />,
      },
    ],
  },
  {
    path: '/todo',
    element: <Layout />,
    children: [
      {
        path: '',
        element: <Todo />,
      },
    ],
  },
  {
    path: '/message',
    element: <Layout />,
    children: [
      {
        path: '',
        element: <Message />,
      },
    ],
  },
  {
    path: '/me',
    element: <Layout />,
    children: [
      {
        path: '',
        element: <PersonalCenter />,
      },
    ],
  },
  // {
  //   path: '/',
  //   element: <Navigate to='/login' />,
  // },
  // {
  //   path: '/login',
  //   element: <Login />,
  //   meta: {
  //     requiresAuth: false,
  //     title: '登录页',
  //     key: 'login',
  //   },
  // },
  {
    path: '*',
    element: <Navigate to='/404' />,
  },
];

export default function Router() {
  const router = createBrowserRouter(rootRouter);

  return <RouterProvider router={router} />;
}
