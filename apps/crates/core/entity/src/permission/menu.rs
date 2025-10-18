//! 菜单表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use database::utils::GenericTreeTrait;

/// 菜单表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_perm_menu")]
pub struct Model {
    /// 菜单ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    pub title: String,
    /// Icon图标类
    pub icon_class: Option<String>,
    /// 菜单类型(0:菜单,1:按钮)
    pub menu_type: i8,
    /// 打开方式(0:组件,1:内链,2:外链)
    pub open_method: i8,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component_path: Option<String>,
    /// 路由重定向
    pub redirect_to: Option<String>,
    /// 链接地址:站内链地址/站外链地址
    pub link: Option<String>,
    /// 链接跳转方式, _blank/_self
    pub link_target: Option<String>,
    /// 是否隐藏
    pub is_hidden: Option<bool>,
    /// 是否始终显示根菜单
    pub is_always_show_root: Option<bool>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now().naive_local());
        Ok(self)
    }
}

/// 实现 `GenericTreeTrait` trait, 将列表数据转换为树结构
impl GenericTreeTrait for Model {
    fn id(&self) -> i32 {
        self.id
    }

    fn pid(&self) -> Option<i32> {
        self.pid
    }
}

pub mod enums {
    use super::*;

    /// 菜单类型
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum MenuType {
        /// 菜单
        Menu = 0,
        /// 按钮
        Button = 1,
    }

    /// 菜单打开方式
    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(i8)]
    pub enum OpenMethod {
        /// 组件
        Component = 0,
        /// 内链
        InternalLink = 1,
        /// 外链
        ExternalLink = 2,
    }

    /// 菜单链接跳转方式
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum LinkTarget {
        /// 新窗口中打开
        #[serde(rename = "_blank")]
        Blank,
        /// 当前窗口中打开
        #[serde(rename = "_self")]
        Current,
    }

    impl From<LinkTarget> for String {
        fn from(value: LinkTarget) -> Self {
            match value {
                LinkTarget::Blank => "_blank".to_owned(),
                LinkTarget::Current => "_self".to_owned(),
            }
        }
    }
}
