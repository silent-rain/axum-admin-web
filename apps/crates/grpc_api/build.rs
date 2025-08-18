use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // trigger rebuild if "proto" folder change
    print!("cargo:rerun-if-changed=./proto");

    //let out_dir = PathBuf::from(env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("proto");
    std::fs::create_dir_all(&out_dir)?;

    tonic_prost_build::configure()
        // 现实启用当 .proto 文件变化时自动重编译
        .emit_rerun_if_changed(true)
        // 生成 gRPC 服务端代码，默认为 true
        .build_server(true)
        // 生成 gRPC 客户端代码，默认为 true
        .build_client(true)
        // 生成描述符文件，当使用 gRPC Reflection 功能时可以从这个文件中获取服务描述信息来返回给调用方
        .file_descriptor_set_path(out_dir.join("getting_descriptor.bin"))
        // 输出目录
        .out_dir(out_dir)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .build_transport(true)
        .compile_protos(
            &["proto/helloworld.proto", "proto/template.proto"], // Update the path to the .proto file
            &["proto"], // Update the search path for proto files
        )?;
    Ok(())
}
