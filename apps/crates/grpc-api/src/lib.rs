pub mod api {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    // tonic::include_proto!("api");
    include!("proto/api.rs");
}

pub mod helloworld {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    // tonic::include_proto!("helloworld");
    include!("proto/helloworld.rs");
}

pub const FILE_DESCRIPTOR_SET: &[u8] =
    //tonic::include_file_descriptor_set!("proto_descriptor");
    include_bytes!("proto/proto_descriptor.bin");
