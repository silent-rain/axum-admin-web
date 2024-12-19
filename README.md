# Axum-Admin-Web

这个是一个后端接口服务，同时内嵌静态文件服务。

## 框架技术栈

### 后端

- 语言: Rust
- 后端框架：Actix-web
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
- [ ] 内嵌 Web 服务
- [ ] 日志
  - [ ] 终端日志
  - [ ] 文件日志
  - [ ] 数据库日志
  - [ ] OpenTelemetry 日志
- [ ] 自定义业务状态码
- [ ] 数据库
  - [ ] 迁移库表
  - [ ] 读写数据库
  - [ ] mock 单元测试
- [x] 依赖注入
- [ ] 中间件
  - [x] 跨域
  - [ ] Actix Request Identifier
  - [ ] Tracing Logger
  - [ ] 系统接口鉴权
  - [ ] OpenApi 鉴权
  - [ ] 接口权限鉴权
  - [ ] 访问频率限制
  - [ ] 访问 IP 限制
  - [ ] 接口操作日志
- [ ] API 文档
  - [ ] ApiPost 接口工具
  - [ ] 内置接口文档
- [ ] 插件
  - [ ] 服务启动 Logo
  - [ ] 请求参数校验插件
  - [ ] [pprof]性能剖析工具
  - [ ] [Prometheus] 指标记录
  - [ ] [Swagger]接口文档, apipost 工具代替
  - [ ] 服务启动后打开浏览器
- [ ] 动态 SEO 优化
- [ ] 内存缓存
- [ ] 订阅
- [ ] cron定时任务
  - [ ] 定时任务调度
    - [ ] 即时任务
    - [ ] 定时任务
  - [ ] 系统任务
  - [ ] 用户任务
- [ ] [websocket]实时通讯

## 业务列表

- [ ] 认证管理
  - [ ] 获取验证码
  - [ ] 用户注册
  - [ ] 登陆
    - [ ] 单点登录
  - [ ] 登出
- [ ] 用户管理
  - [ ] 角色管理
  - [ ] 用户信息管理
  - [ ] 用户手机号管理
  - [ ] 用户邮箱管理
  - [ ] 用户区块链钱包管理
  - [ ] 会员等级管理
  - [ ] 用户地理位置管理
- [ ] 权限管理
  - [ ] 菜单管理
  - [ ] 令牌管理
  - [ ] OpenApi 接口管理
- [ ] 组织管理
  - [ ] 部门管理
  - [ ] 岗位管理
  - [ ] 职级管理
- [ ] 系统管理
  - [ ] 配置管理
  - [ ] 图片验证码管理
  - [ ] 图片资源管理
  - [ ] 数据字典管理
- [ ] 任务调度作业管理
  - [ ] 任务调度作业
  - [ ] 任务调度状态日志
  - [ ] 任务调度事件日志
- [ ] 日志管理
  - [ ] 系统日志
  - [ ] 操作日志
  - [ ] 前端日志
  - [ ] 登陆日志
- [ ] 前端权限
  - [ ] 动态路由
  - [ ] 按钮权限
- [ ] 系统监控

## 待办

- 调研 log span 全局使用事件，放置在中间件中，这样日志中可以获取统一的 span 数据；
- 定时器-用户任务
- doc 文档，细化为接口文档；
- 用户权限封装；

- <https://github.com/actix/actix-extras/tree/master/actix-identity>
- <https://crates.io/crates/actix-ws>
- <https://crates.io/crates/actix-session>
- <https://github.com/actix/actix-extras/tree/master/actix-limitation>
- <https://github.com/ddimaria/rust-actix-example>

## 开发文档

- [开发环境搭建](./docs/开发环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [Sea-Orm 使用指南](./docs/Sea-Orm使用指南.md)
- [问题答疑](./docs/Q&A.md)
