# Sccache编译加速 - 共享缓存

sccache 是一个三方工具，可以用于在不同的工作空间中共享已经构建好的依赖包。

## 安装

```shell
cargo install sccache
```

## 配置

### 配置方案1

> vim .bashrc

```shell
RUSTC_WRAPPER = /path/to/sccache
```

### 配置方案2
>
> vim .bashrc

```toml

[build]
rustc-wrapper = "…"           # 使用该 wrapper 来替代 rustc
```
