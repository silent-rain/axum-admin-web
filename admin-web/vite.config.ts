import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'


// https://vite.dev/config/
export default defineConfig({
  server: {
    host: 'localhost', // 指定监听的IP地址
    port: 1420, // 指定服务器端口
    open: false, // 开发服务器启动时，自动在浏览器打开
    strictPort: false, // 若端口已被占用，就尝试下一个可用端口
    cors: true, // 允许跨域
    // 配置代理
    proxy: {
      '/api': {
        target: 'http://127.0.0.1: 8000', // 接口地址
        changeOrigin: true, // 接口跨域
        secure: false, // 启用 https 服务时需要配置
      },
    },
  },
  plugins: [
    react(),
  ],
  base: './',
  publicDir: 'public',
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  },
  build: {
    target: 'modules', // 浏览器兼容目标
    outDir: 'dist', // 打包输出路径
    assetsDir: 'assets', // 静态资源存放路径
    cssCodeSplit: true, // 允许 css 代码拆分
    sourcemap: false, // 不生成 sourceMap 文件
    minify: 'terser', // 缩小文件体积
    terserOptions: {
      compress: {
        drop_console: true, // 取消 console
        drop_debugger: true, // 取消 debugger
      },
    },
  },
})
