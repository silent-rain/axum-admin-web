//! 菜单管理

use entity::permission::menu;

use actix_validator::Validate;

use serde::{Deserialize, Serialize};

/// 查询菜单列表
#[derive(Default, Deserialize, Validate)]
pub struct GetMenuListReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 菜单名称
    pub title: Option<String>,
    /// 返回所有数据
    pub all: Option<bool>,
}

/// 添加菜单
#[derive(Serialize, Deserialize, Validate)]
pub struct AddMenuReq {
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub title: String,
    /// Icon图标类
    pub icon_class: Option<String>,
    /// 菜单类型(0:菜单,1:按钮)
    pub menu_type: menu::enums::MenuType,
    /// 打开方式(0:组件,1:内链,2:外链)
    pub open_method: menu::enums::OpenMethod,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component_path: Option<String>,
    /// 路由重定向
    pub redirect_to: Option<String>,
    /// 链接地址:站内链地址/站外链地址
    pub link: Option<String>,
    /// 链接跳转方式, _blank/_self
    pub link_target: Option<menu::enums::LinkTarget>,
    /// 是否隐藏(0:显示,1:隐藏)
    pub is_hidden: Option<menu::enums::IsHidden>,
    /// 是否始终显示根菜单(0:隐藏,1:显示)
    pub is_always_show_root: Option<menu::enums::IsAlwaysShowRoot>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: menu::enums::Status,
}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuReq {
    /// 父菜单ID
    pub pid: Option<i32>,
    /// 菜单名称
    #[validate(length(min = 2, message = "至少输入两个字符"))]
    pub title: String,
    /// Icon图标类
    pub icon_class: Option<String>,
    /// 菜单类型(0:菜单,1:按钮)
    pub menu_type: menu::enums::MenuType,
    /// 打开方式(0:组件,1:内链,2:外链)
    pub open_method: menu::enums::OpenMethod,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component_path: Option<String>,
    /// 路由重定向
    pub redirect_to: Option<String>,
    /// 链接地址:站内链地址/站外链地址
    pub link: Option<String>,
    /// 链接跳转方式, _blank/_self
    pub link_target: Option<menu::enums::LinkTarget>,
    /// 是否隐藏(0:显示,1:隐藏)
    pub is_hidden: Option<menu::enums::IsHidden>,
    /// 是否始终显示根菜单(0:隐藏,1:显示)
    pub is_always_show_root: Option<menu::enums::IsAlwaysShowRoot>,
    /// 权限标识
    pub permission: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(0:停用,1:正常)
    pub status: menu::enums::Status,
}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuStatusReq {
    /// 状态(0:停用,1:正常)
    pub status: menu::enums::Status,
}
