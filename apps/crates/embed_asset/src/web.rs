//! 资源内嵌

use rust_embed::{EmbeddedFile, RustEmbed};

use crate::asset::EmbedAssetTrait;

/// 后台管理 WEB 静态资源
#[derive(Debug, Clone, Default, RustEmbed)]
// #[folder = "../../../admin-web/dist/"]
#[folder = "../../admin/dist/"]
pub struct AssetAdminWebDist;

impl EmbedAssetTrait for AssetAdminWebDist {
    fn get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let asset = AssetAdminWebDist.data("index.html");
        assert!(asset.is_some());

        let asset = AssetAdminWebDist.data("/index.html");
        assert!(asset.is_none());

        let asset = AssetAdminWebDist.data("./index.html");
        assert!(asset.is_none());

        // let asset = AssetAdminWebDist.data("assets/index-CilDdteP.css");
        // assert!(asset.is_some());
    }
}
