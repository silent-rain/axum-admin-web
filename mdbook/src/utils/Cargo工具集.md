# Cargo工具集

## cargo tree

可以显示项目的依赖树，并且可以帮助你识别哪些依赖项被实际使用。
虽然它不能直接告诉你哪些依赖是未使用的，但通过查看依赖树，你可以手动检查哪些依赖可能未被使用。

```shell
# 运行
cargo tree
```

## cargo-udeps

是一个专门用于查找未使用依赖的工具。你可以通过以下步骤安装和使用它：

```shell
# 安装
cargo install cargo-udeps

# 运行
cargo +nightly udeps
```

注意：cargo-udeps 需要 nightly 版本的 Rust 编译器。

## cargo-outdated

可以帮助你检查过时的依赖项，但它也可以间接地帮助你识别未使用的依赖项，因为过时的依赖项可能是未使用的。

```shell
# 安装
cargo install cargo-outdated

# 运行
cargo outdated
```

## cargo-asm

cargo-asm 可以显示 Rust 函数的汇编代码，这可以帮助你确定哪些函数和模块被实际使用。虽然这种方法比较繁琐，但对于某些复杂的项目，它可能是一个有用的工具。

安装
cargo-asm

```shell
# 安装
cargo install cargo-asm

# 运行
cargo asm <function_name>
```
