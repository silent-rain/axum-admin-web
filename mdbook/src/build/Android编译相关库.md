# Android编译相关库

```toml
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
libc = "0.2"
log = "0.4"
ndk = { version = "0.9", features = ["api-level-30"] }
ndk-context = "0.1"
android_logger = "0.14"
android-activity = { version = "0.6", features = ["native-activity"] }

[target.'cfg(target_family = "wasm")'.dependencies]
console_error_panic_hook = "0.1"
wasm-logger = "0.2"
```
