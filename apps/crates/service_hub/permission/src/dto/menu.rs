//! 菜单管理

use entity::permission::menu;

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 查询菜单列表
#[derive(Default, Deserialize, Validate)]
pub struct GetMenusReq {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMenusResp {
    pub data_list: Vec<menu::Model>,
    pub total: u64,
}

/// 查询菜单详情 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetMenuReq {
    /// 菜单ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMenuResp {
    #[serde(flatten)]
    data: menu::Model,
}

/// 添加菜单
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateMenuReq {
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
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMenuResp {}

/// 更新数据
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuReq {
    /// 菜单ID
    pub id: i32,
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
    /// 状态(false:停用,true:正常)
    pub status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuResp {}

/// 更新数据状态
#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMenuStatusReq {
    /// 菜单ID
    pub id: i32,
    /// 状态(false:停用,true:正常)
    pub status: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuStatusResp {}

/// 删除菜单 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteMenuReq {
    /// 菜单ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMenuResp {}

/// 获取子菜单列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetMenuChildrenReq {
    /// 父菜单ID
    pub pid: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMenuChildrenResp {
    pub data_list: Vec<menu::Model>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuTreeItem {
    #[serde(flatten)]
    pub data: menu::Model,
    pub children: Vec<MenuTreeItem>,
}

/// 菜单树列表 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct GetMenuTreeReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMenuTreeResp {
    #[serde(flatten)]
    pub data: MenuTreeItem,
}
