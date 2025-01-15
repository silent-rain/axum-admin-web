import CryptoJS from 'crypto-js';

const sign = '0102030405060708';

/**
 * 密码加密
 */
export function passwordEncryption(passwd: string) {
  //密码加密
  const iv = CryptoJS.enc.Utf8.parse(sign);
  const key = CryptoJS.enc.Utf8.parse(sign);
  const src = CryptoJS.enc.Utf8.parse(passwd);
  const aes = CryptoJS.AES.encrypt(src, key, {
    iv: iv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.Pkcs7,
  });
  //
  return aes.toString();
}
