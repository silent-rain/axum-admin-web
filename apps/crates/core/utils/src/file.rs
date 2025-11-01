//! 文件处理

use std::path::Path;

use crate::error::Error;

/// 获取文件扩展名
pub fn file_extension(filename: String) -> Result<String, Error> {
    let path = Path::new(&filename);
    let extension = path
        .extension()
        .ok_or_else(|| Error::ParseFileExtension("not found extension".to_string()))?
        .to_str()
        .ok_or_else(|| Error::ConvertType("OsStr convert str failed".to_string()))?
        .to_string();

    Ok(extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extension() -> Result<(), Error> {
        let extension = file_extension("demo.txt".to_string())?;
        println!("extension: {:#?}", extension);
        assert!(extension == "txt");
        Ok(())
    }
}
