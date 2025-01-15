# QA

## 编译时提示 To load an ES module, set "type": "module" in the package.json or use the .mjs extension

- 在`package.json`已经配置后, 还是有这个提示

```shell
pnpm build                      

> react-mobile@0.0.0 build /home/one/code/react-mobile
> tsc -b && vite build

vite v6.0.2 building for production...
transforming (8) node_modules/.pnpm/react@18.3.1/node_modules/react/cjs/react-jsx-runtime.production.min.(node:1545) Warning: To load an ES module, set "type": "module" in the package.json or use the .mjs extension.
(Use `node --trace-warnings ...` to show where the warning was created)
(node:1545) Warning: To load an ES module, set "type": "module" in the package.json or use the .mjs extension.
✓ 42 modules transformed.
dist/index.html                   0.45 kB │ gzip:  0.30 kB
dist/assets/index-JHIJg_Ga.css    6.66 kB │ gzip:  1.94 kB
dist/assets/index-CUrVhwO9.js   212.07 kB │ gzip: 68.94 kB
✓ built in 2.77s
```

解决方案:
将`tailwind.config.js`修改为`tailwind.config.mjs`.

## 别名识别异常问题

需要在 `tsconfig.app.json` 中添加别名配置

```js
{
  "compilerOptions": {
    // 路径映射配置
    "baseUrl": "./",
    "paths": {
      "@": [
        "src"
      ],
      "@/*": [
        "src/*"
      ],
    }
  },
}
```

## ESLint插件在VSCode中不提示、不生效

- [ESLint插件在VSCode中不提示、不生效](https://blog.csdn.net/weixin_51705477/article/details/136321745)

## 要修复“规则@tailwind 处未知”警告

- [如何修复 Tailwind CSS 中的 unknownAtRules 警告](https://www.codeconcisely.com/posts/tailwind-css-unknown-at-rules/#installing-tailwind-css-intellisense-plugin)
- 请先安装Tailwind CSS IntelliSense插件
- 按Shift + P并搜索“更改语言模式”
- 在搜索栏中，输入“ tailwindcss ”并选择它。
