//! 权限相关
pub mod menu;
pub mod menu_role_rel;
pub mod openapi;
pub mod openapi_role_rel;
pub mod token;
pub mod token_role_rel;

pub use menu::Entity as MenuEntity;
pub use menu_role_rel::Entity as MenuRoleRelEntity;
pub use openapi::Entity as OpenapiEntity;
pub use openapi_role_rel::Entity as OpenapiRoleRelEntity;
pub use token::Entity as TokenEntity;
pub use token_role_rel::Entity as TokenRoleRelEntity;
