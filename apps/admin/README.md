# 后台管理-后端

这个是一个后端接口服务，同时内嵌静态文件服务。

## 技术栈

- axum
- tokio
- Tower

## 编译与调试

### 代码检查

```shell
cargo clippy
# or
cargo clippy -p admin
```

### 调试模式

```shell
cd apps/admin

cargo run -p admin
```

### 生产模式

```shell
cd apps/admin

cargo build -p admin
```

### 查询依赖

```shell
cd apps/admin

cargo +nightly udeps -p admin

cargo +nightly udeps --workspace
```

## 开发文档

```shell
cd apps/admin

cargo doc -p admin
```

## 服务

- [前端服务](http://127.0.0.1:8000/)
- [后端服务](http://127.0.0.1:8000/api/v1/)
- [ ][swagger-ui](http://127.0.0.1:8000/swagger-ui/)

## 相关文档

- [axum - 生态系统](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md)

- axum错误处理
  - [自定义 extractor](https://www.cnblogs.com/pythonClub/p/17804708.html)
  - [Axum的错误处理模块](https://yuxuetr.com/wiki/axum/axum-error-handling)
  - [axum中的各种响应](https://www.cnblogs.com/pythonClub/p/17804749.html)
  - [Axum的response模块](https://yuxuetr.com/wiki/axum/axum-response)

- axum中间件
  - [Axum中如何实现中间件？](https://yuxuetr.com/blog/2024/06/09/axum-middleware)
  - [Axum的middleware模块](https://yuxuetr.com/wiki/axum/axum-middleware)

- 数据库
  - [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)

- 参数校验
  - [validation](https://dev.to/chaudharypraveen98/form-validation-in-rust-404l)
  - [validator](https://lib.rs/crates/validator)
