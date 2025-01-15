# Axum-Admin-Web

这个是一个后端接口服务，同时内嵌静态文件服务。

## 框架技术栈

### 后端

- 语言: Rust
- 后端框架: Axum
- 数据库框架: Sea-Orm
- 日志框架: Tracing

### 前端

- 语言：
- 构建工具: Vite
- UI 框架：

## 框架功能列表

### 后端框架

后端框架功能列表。

- [x] 热重启
- [x] 内嵌 Web 服务
- [x] 日志
  - [x] 终端日志
  - [x] 文件日志
  - [x] 数据库日志
  - [ ] [OpenTelemetry 日志](https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk)
- [x] 自定义业务状态码
- [x] 数据库
  - [x] 迁移库表
  - [x] 读写数据库
  - [x] mock 单元测试
- [x] 依赖注入
- [ ] 中间件
  - [x] 跨域
  - [x] Axum Request Id
  - [x] Tracing 高级跟踪/记录
  - [x] 自动压缩响应
  - [x] Timeout 超时
  - [x] 系统接口鉴权
  - [x] OpenApi 鉴权
  - [x] Casbin接口权限鉴权
  - [x] 访问频率限制
  - [ ] 访问 IP 限制
  - [ ] 接口操作日志
- [ ] API 文档
  - [ ] ApiPost 接口工具
  - [ ] 内置接口文档
- [ ] 插件
  - [ ] 服务启动 Logo
  - [x] 请求参数校验插件
  - [ ] [pprof]性能剖析工具
  - [x] [Prometheus] 指标记录
  - [ ] [Swagger]接口文档, apipost 工具代替
  - [ ] 服务启动后打开浏览器
- [ ] 动态 SEO 优化
- [x] 内存缓存
- [ ] 订阅
- [x] cron定时任务
  - [x] 定时任务调度
    - [x] 即时任务
    - [ x] 定时任务
  - [x] 系统任务
  - [x] 用户任务
- [ ] [websocket]实时通讯

## 业务列表

- [x] 认证管理
  - [x] 获取验证码
  - [x] 用户注册
  - [x] 登陆
    - [ ] 单点登录
  - [x] 登出
- [x] 用户管理
  - [x] 角色管理
  - [x] 用户信息管理
  - [x] 用户手机号管理
  - [x] 用户邮箱管理
  - [x] 用户区块链钱包管理
  - [x] 会员等级管理
  - [x] 用户地理位置管理
  - [x] 登陆日志
  - [ ] session 登录态管理
- [x] 权限管理
  - [x] 菜单管理
  - [x] 令牌管理
  - [x] OpenApi 接口管理
- [x] 组织管理
  - [x] 部门管理
  - [x] 岗位管理
  - [x] 职级管理
- [x] 系统管理
  - [x] 配置管理
  - [x] 图片验证码管理
  - [x] 图片资源管理
  - [x] 数据字典管理
- [x] 任务调度作业管理
  - [x] 任务调度作业
  - [x] 任务调度状态日志
  - [x] 任务调度事件日志
- [x] 日志管理
  - [x] 系统日志
  - [x] 操作日志
  - [x] 前端日志
- [ ] 前端权限
  - [ ] 动态路由
  - [ ] 按钮权限
- [ ] 系统监控

## 待办

- 下载文件:
  - <https://github.com/tokio-rs/axum/discussions/608>
- 如何托管 SPA 文件并将文件嵌入到可执行文件中？
  - <https://github.com/tokio-rs/axum/discussions/1309>
- curd 设计参考
  - <https://github.com/zenlex/crustd/blob/main/src/crud_traits.rs>
- 定时器-用户任务
- doc 文档，细化为接口文档；
- 用户权限封装；
- 文件上传表
  - 图片展示接口验证
  - 添加文件下载接口
- session 登录态管理
  - 结合登录日志进行管理
  - 封装自定义session库
  - 登陆后设置session，设置cookies
  - 上下文初始化，仅读取session
  - 鉴权，仅读取session；
- 日志响应body异常

## 开发文档

- [开发环境搭建](./docs/开发环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [Sea-Orm 使用指南](./docs/Sea-Orm使用指南.md)
- [问题答疑](./docs/Q&A.md)
