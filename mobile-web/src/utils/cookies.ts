import Cookies from 'js-cookie';

// 设置cookie
export const setCookies = (name: string, value: any, expires = undefined, path = '') => {
  // 名称、内容、有效期 若无失效时间，网站关闭时失效、路径
  Cookies.set(name, value, { expires, path });
};

// 获取cookie
export const getCookies = (name: string) => {
  return Cookies.get(name);
};

// 删除cookie
export const removeCookies = (name: string, expires = undefined, path = '') => {
  return Cookies.remove(name, { expires, path });
};
