import axios, { AxiosError, AxiosRequestConfig, AxiosResponse } from 'axios';
import { Modal } from 'antd-mobile';

function dispatchLogout() {
  Modal.confirm({
    content: '你已被登出, 请重新登录',
    onConfirm: () => {
      // TODO 清除登录信息
      //   store.dispatch<any>(logout());
    },
  });
}

// 创建axios的实例
const service = axios.create({
  baseURL: process.env.VITE_BASE_API,
  timeout: 5000,
});

// 请求拦截器
service.interceptors.request.use(
  (config) => {
    // TODO 请求携带凭证
    // const token = store.getState().user.token;
    // if (token) {
    //   config.headers.Authorization = 'Bearer ' + token;
    // }
    return config;
  },
  (error) => Promise.reject(error),
);

// 响应拦截器
service.interceptors.response.use(
  (response) => {
    const res = response.data;

    if (res.code !== 20000) {
      Modal.show({
        content: res.message || 'Error',
        closeOnMaskClick: true,
      });
      // 50008：非法token，50012：其他客户端登录，50014：token失效了
      if (res.code === 50008 || res.code === 50012 || res.code === 50014) {
        // 尝试重新登录
        dispatchLogout();
      }
      return Promise.reject(res.message || 'Error');
    } else {
      return response;
    }
  },
  (error: AxiosError) => {
    const response = error.response;
    const data = response && (response.data as any);
    if (response) {
      switch (response.status) {
        case 400:
          error.message = data.message || '错误请求';
          break;
        case 401:
          error.message = data.message || 'token失效，请重新登录';
          break;
        case 403:
          error.message = data.message || '非法token，拒绝访问';
          dispatchLogout();
          break;
        case 404:
          error.message = data.message || '请求错误，资源找不到了';
          break;
        case 408:
          error.message = data.message || '请求超时';
          break;
        case 500:
          error.message = data.message || '服务器错误';
          break;
        default:
          error.message = data.message || '连接错误';
      }
    } else {
      if (!window.navigator.onLine) {
        error.message = '网络中断';
      }
    }
    Modal.show({
      content: error.message,
      closeOnMaskClick: true,
    });
    return Promise.reject(error);
  },
);

/** 封装request请求方法 */
function request<T = any>(config: AxiosRequestConfig) {
  return service.request<any, AxiosResponse<T>>(config);
}

export default request;
