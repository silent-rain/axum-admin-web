// GRPC 接口模块

pub const FILE_DESCRIPTOR_SET: &[u8] =
    //tonic::include_file_descriptor_set!("getting_descriptor");
    include_bytes!("proto/getting_descriptor.bin");

pub mod helloworld {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    // tonic::include_proto!("helloworld");
    include!("proto/helloworld.rs");
}

pub mod template {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    include!("proto/template.rs");
}
