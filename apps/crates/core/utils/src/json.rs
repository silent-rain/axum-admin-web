//! JSON 序列化与反序列化转换

use code::Error;

use serde::{Deserialize, Deserializer, Serializer, de::DeserializeOwned};
use tracing::error;

/// 反序列化 vec 转 string
pub fn vec_to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, Error> {
    let v: Vec<String> = Deserialize::deserialize(deserializer).map_err(|err| {
        error!("failed to deserialize vec into string, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;

    let result = serde_json::to_string(&v).map_err(|err| {
        error!("failed to serialize vec into string, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })?;
    Ok(result)
}

/// 序列化 i8 转 bool
pub fn i8_to_bool<S: Serializer>(v: &i8, serializer: S) -> Result<S::Ok, Error> {
    serializer.serialize_bool(*v != 0).map_err(|err| {
        error!("failed to serialize i8 into bool, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })
}

/// 反序列化 bool 转 i8
pub fn bool_to_i8<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i8, Error> {
    let b: bool = Deserialize::deserialize(deserializer).map_err(|err| {
        error!("failed to deserialize bool into i8, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;
    Ok(if b { 1 } else { 0 })
}

/// 将一个结构体转换为另一个结构体
pub fn struct_to_struct<S, T>(src: &S) -> Result<T, Error>
where
    S: serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    // 转换为JSON字符串
    let data = serde_json::to_string(src).map_err(|err| {
        error!("serialize to JSON string failed, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })?;

    // 将JSON字符串反序列化为结构体
    let target: T = serde_json::from_str(&data).map_err(|err| {
        error!("failed to deserialize JSON string into struct, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;
    Ok(target)
}

/// 将一个结构体转换为另一个结构体
pub fn vec_deserialize<T>(src: &[u8]) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    // 将JSON字符串反序列化为结构体
    let target: T = serde_json::from_slice(src).map_err(|err| {
        error!("failed to deserialize vec into T, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;
    Ok(target)
}

/// 将一个结构体转换为vec
pub fn bincode_struct_to_vec<S>(src: &S) -> Result<Vec<u8>, Error>
where
    S: serde::Serialize + bincode::Encode,
{
    let config = bincode::config::standard();

    let encoded: Vec<u8> = bincode::encode_to_vec(&src, config).map_err(|err| {
        error!("serialize to JSON byte vector failed, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })?;

    Ok(encoded)
}

/// 将数据序列化
pub fn serialize_to_vec<S>(src: &S) -> Result<Vec<u8>, Error>
where
    S: serde::Serialize,
{
    let target: Vec<u8> = serde_json::to_vec(src).map_err(|err| {
        error!("serialize to JSON byte vector failed, error: {err:#?}");
        Error::JsonSerialization(err.to_string())
    })?;
    Ok(target)
}

// 可以将 Vec<u8> 反序列化为任何实现了 Deserialize 特性的类型
pub fn deserialize_from_vec<T>(data: Vec<u8>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let target: T = serde_json::from_slice(&data).map_err(|err| {
        error!("failed to deserialize vec into struct, error: {err:#?}");
        Error::JsonDeserialization(err.to_string())
    })?;
    Ok(target)
}
