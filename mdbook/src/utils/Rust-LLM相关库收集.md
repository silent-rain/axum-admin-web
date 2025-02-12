# Rust-LLM相关库收集

## candle

```toml
[dependencies]
# candle-datasets = "0.8"      # 不支持 android
# candle-flash-attn = "0.8"    # 不支持 android
# candle-kernels = "0.8"       # 不支持 android, need CUDA installed
# candle-metal-kernels = "0.8" # 不支持 android
candle-core = "0.8"
candle-nn = "0.8"
candle-onnx = "0.8"
candle-transformers = "0.8"
hf-hub = "0.3"              # 与 huggingface hub 集成
tokenizers = "0.21"         # 标记文本
parquet = "53.2"            # 列式存储数据文件格式
# image = "0.25"              # 图像处理库 不支持 android

[build-dependencies]
# cc = "1.1"
bindgen_cuda = { version = "0.1", optional = true }

[features]
default = []
cuda = [
    "candle-core/cuda",
    "candle-nn/cuda",
    "candle-transformers/cuda",
    "dep:bindgen_cuda",
]
cudnn = ["candle-core/cudnn"]
# flash-attn = ["cuda", "candle-transformers/flash-attn", "dep:candle-flash-attn"]
metal = ["candle-core/metal", "candle-nn/metal"]
```

## burn

```toml
# burn = "0.13"
# https://docs.rs/crate/burn-tch/0.13.2
# burn-tch = "0.13"
```

## torch

```toml
# https://github.com/LaurentMazare/tch-rs
# tch = "0.15"
# torch-sys = "0.17.0"
```

## llama.cpp 绑定

```toml

## llama.cpp 绑定
# drama_llama = "0.5" # android 编译失败

# encoding_rs = "0.8.34"
# llama-cpp-2 = "0.1.83" # 原始绑定使用较复杂, android 编译失败
# llama-cpp-2 = { path = "llama-cpp-rs/llama-cpp-2" } # 本地依赖, 最新 llama.cpp android 编译失败
# llama-cpp-2 = { git = "https://github.com/utilityai/llama-cpp-rs.git" } # 魔改后可以支持 android 编译

# llama_cpp = "0.3.2"
# llama_cpp = { git = "https://github.com/vargad/llama_cpp-rs.git", branch = "bump_3038" } // 最新的版本, 运行失败 llama.cpp 可能过低

# llama_cpp_rs = "0.3" # 需要手工微调, 运行失败 llama.cpp 可能过低
# llama_cpp_rs = { path = "rust-llama.cpp" } # 本地依赖, 最新 llama.cpp 编译失败
```

## llama-cpp-2

### 拉取代码与引入项目

- 克隆仓库

```shell
git clone https://github.com/utilityai/llama-cpp-rs.git
```

- 本地引入项目

```shell
llama-cpp-2 = { path = "llama-cpp-rs/llama-cpp-2" } # 本地依赖, 最新 llama.cpp android 编译失败
```

### 魔改编译脚本

```rust
// build.rs

// 打印编译目标信息
let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
println!("target_arch: {target_arch}");
println!("target_os: {target_os}");

if target_arch == "aarch64" {
    config.define(
        "CMAKE_TOOLCHAIN_FILE",
        "/home/one/Android/Sdk/ndk/28.0.12433566/build/cmake/android.toolchain.cmake",
    );
    config.define("ANDROID_PLATFORM", "android-35");
    config.define("ANDROID_ABI", "arm64-v8a");
    config.define("CMAKE_C_FLAGS", "-march=armv8.4a+dotprod");
}

// if target_arch == "arm" { // 旧的编译脚本, ok
//     config.define(
//         "CMAKE_TOOLCHAIN_FILE",
//         "/home/one/Android/Sdk/ndk/28.0.12433566/build/cmake/android-legacy.toolchain.cmake",
//     );
//     config.define("ANDROID_PLATFORM", "android-35");
// }

if target_arch == "arm" {
    config.define(
        "CMAKE_TOOLCHAIN_FILE",
        "/home/one/Android/Sdk/ndk/28.0.12433566/build/cmake/android.toolchain.cmake",
    );
    config.define("ANDROID_PLATFORM", "android-35");
    config.define("ANDROID_ABI", "armeabi-v7a");
    config.define(
        "CMAKE_C_FLAGS",
        // "-march=armv7-a -mfpu=vfpv3-d16 -mfloat-abi=hard",
        "-march=armv7-a -mfpu=vfpv3-d16",
    );
}

if target_arch == "x86_64" && target_os == "android" {
    config.define(
        "CMAKE_TOOLCHAIN_FILE",
        "/home/one/Android/Sdk/ndk/28.0.12433566/build/cmake/android.toolchain.cmake",
    );
    config.define("ANDROID_PLATFORM", "android-35");
    config.define("ANDROID_ABI", "x86_64");
    config.define("CMAKE_C_FLAGS", "-march=x86-64");
}

```

## Android NDK构建llama.cpp项目

```shell
# 代码准备，我担心影响上边编译好的，重新拉了一份代码，其实是不影响的
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp

#开始构建
mkdir build-android
cd build-android

# 查看你的ndk文件夹路径
export NDK=<your_ndk_directory>

cmake -DCMAKE_TOOLCHAIN_FILE=$NDK/build/cmake/android.toolchain.cmake -DANDROID_ABI=arm64-v8a -DANDROID_PLATFORM=android-23 -DCMAKE_C_FLAGS=-march=armv8.4a+dotprod ..

make
```

## 其他LLM库

```toml
# kalosm = "0.3" # 不支持 android
# callm = "0.2" # 不支持 android
```
