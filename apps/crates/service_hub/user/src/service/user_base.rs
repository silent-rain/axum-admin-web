//! 用户信息管理
use crate::{
    dao::{user_base::UserBaseDao, user_role_rel::UserRoleRelDao},
    dto::user_base::{
        CreateUserBaseReq, DeleteUserBaseReq, GetCheckUsernameReq, GetUserBaseReq, GetUserBasesReq,
        ProfileResp, RolesReq, UpdateShareCodeReq, UpdateUserBaseReq, UpdateUserBaseStatusReq,
    },
};

use code::{Error, ErrorMsg};
use entity::user::{role, user_base, user_role_rel};

use base64::Engine;
use nject::injectable;
use sea_orm::Set;
use tracing::error;
use utils::crypto::sha2_256;
use uuid::Uuid;

/// 用户分享码生成次数
const SHARE_CODE_COUNT: i32 = 100;

/// 服务层
#[injectable]
pub struct UserBaseService {
    user_base_dao: UserBaseDao,
    user_role_rel_dao: UserRoleRelDao,
}

impl UserBaseService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetUserBasesReq,
    ) -> Result<(Vec<user_base::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.user_base_dao.list(req).await.map_err(|err| {
            error!("查询用户信息列表失败, err: {:#?}", err);
            Error::DbQueryError
                .into_msg()
                .with_msg("查询用户信息列表失败")
        })?;

        // 屏蔽敏感信息
        for result in results.iter_mut() {
            result.password = "".to_string();
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetUserBaseReq) -> Result<user_base::Model, ErrorMsg> {
        let mut result = self
            .user_base_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("user_id: {}, 用户不存在", req.id);
                Error::DbQueryEmptyError.into_msg().with_msg("用户不存在")
            })?;

        // 屏蔽敏感信息
        result.password = "".to_string();
        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateUserBaseStatusReq) -> Result<(), ErrorMsg> {
        self.user_base_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                error!("更新用户信息状态失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新用户信息状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteUserBaseReq) -> Result<u64, ErrorMsg> {
        let result = self.user_base_dao.delete(req.id).await.map_err(|err| {
            error!("删除用户信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_msg().with_msg("删除用户信息失败")
        })?;

        Ok(result)
    }
}

/// 创建/更新用户
///
/// 事务处理
impl UserBaseService {
    /// 后台添加用户信息及对应用户信息的角色
    pub async fn create(&self, req: CreateUserBaseReq) -> Result<user_base::Model, ErrorMsg> {
        // 检查用户名, 查看用户名是否已注册
        self.check_username(GetCheckUsernameReq {
            username: req.username.clone(),
        })
        .await?;

        // 密码加密
        let password = sha2_256(&req.password);

        let model = user_base::ActiveModel {
            username: Set(req.username),
            real_name: Set(req.real_name),
            gender: Set(req.gender as i8),
            password: Set(password),
            status: Set(req.status),
            age: Set(req.age),
            date_birth: Set(req.date_birth),
            avatar: Set(req.avatar),
            intro: Set(req.intro),
            desc: Set(req.desc),
            address: Set(req.address),
            preferences: Set(req.preferences),
            department_id: Set(req.department_id),
            position_id: Set(req.position_id),
            rank_id: Set(req.rank_id),
            member_level_id: Set(req.member_level_id),
            ..Default::default()
        };

        let result = self
            .user_base_dao
            .add_user(model, req.role_ids)
            .await
            .map_err(|err| {
                error!("添加用户信息失败, err: {:#?}", err);
                Error::DbAddError.into_msg().with_msg("添加用户信息失败")
            })?;
        Ok(result)
    }

    /// 后台更新用户信息及对应用户信息的角色
    pub async fn update(&self, req: UpdateUserBaseReq) -> Result<(), ErrorMsg> {
        // 获取原角色列表
        let (user_role_rels, _) = self
            .user_role_rel_dao
            .list_by_user_id(req.id)
            .await
            .map_err(|err| {
                error!("查询用户信息与角色关系列表失败, err: {:#?}", err);
                Error::DbQueryError
                    .into_msg()
                    .with_msg("查询用户信息与角色关系列表失败")
            })?;

        // 获角色色ID的差异列表
        let (add_role_ids, del_role_ids) = self.diff_role_ids(req.role_ids, user_role_rels);

        let model = user_base::ActiveModel {
            id: Set(req.id),
            real_name: Set(req.real_name),
            gender: Set(req.gender as i8),
            status: Set(req.status),
            age: Set(req.age),
            date_birth: Set(req.date_birth),
            avatar: Set(req.avatar),
            intro: Set(req.intro),
            desc: Set(req.desc),
            address: Set(req.address),
            preferences: Set(req.preferences),
            department_id: Set(req.department_id),
            position_id: Set(req.position_id),
            rank_id: Set(req.rank_id),
            member_level_id: Set(req.member_level_id),
            ..Default::default()
        };
        self.user_base_dao
            .update_user(model, add_role_ids, del_role_ids)
            .await
            .map_err(|err| {
                error!("更新用户信息失败, err: {:#?}", err);
                Error::DbUpdateError.into_msg().with_msg("更新用户信息失败")
            })?;

        Ok(())
    }

    /// 获角色色ID的差异列表
    fn diff_role_ids(
        &self,
        role_ids: Vec<i32>,
        user_role_rels: Vec<user_role_rel::Model>,
    ) -> (Vec<i32>, Vec<i32>) {
        let raw_role_ids: Vec<i32> = user_role_rels.iter().map(|v| v.role_id).collect();
        // 待新增的ID
        let mut add_role_ids: Vec<i32> = Vec::new();
        for role_id in role_ids.clone().into_iter() {
            if !raw_role_ids.contains(&role_id) {
                add_role_ids.push(role_id);
            }
        }

        // 待删除的ID
        let mut del_role_ids: Vec<i32> = Vec::new();
        for raw_role_id in raw_role_ids.into_iter() {
            if !role_ids.contains(&raw_role_id) {
                del_role_ids.push(raw_role_id);
            }
        }

        (add_role_ids, del_role_ids)
    }
}

impl UserBaseService {
    /// 更新用户分享码
    pub async fn update_share_code(&self, req: UpdateShareCodeReq) -> Result<(), ErrorMsg> {
        // 获取分享码
        let mut share_code = String::new();
        for _i in 0..SHARE_CODE_COUNT {
            let share_code_uuid = Uuid::new_v4().to_string().replace('-', "");
            let share_code_hash =
                base64::engine::general_purpose::STANDARD.encode(&share_code_uuid);
            let half_share_code = share_code_hash[0..16].to_string();

            // 检查分享码是否存在
            if !self.check_share_code_exist(half_share_code.clone()).await? {
                share_code = half_share_code.to_string();
                break;
            }
        }

        if share_code.is_empty() {
            error!("生成用户分享码失败, 请重试");
            return Err(Error::GenerateUserShareCore
                .into_msg()
                .with_msg("生成用户分享码失败, 请重试"));
        }

        self.user_base_dao
            .update_share_code(req.id, share_code)
            .await
            .map_err(|err| {
                error!("更新用户分享码失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("更新用户分享码失败")
            })?;

        Ok(())
    }

    /// 检查分享码是否存在
    async fn check_share_code_exist(&self, share_code: String) -> Result<bool, ErrorMsg> {
        let result = self
            .user_base_dao
            .info_by_share_code(share_code)
            .await
            .map_err(|err| {
                error!("查询用户分享码失败, err: {:#?}", err);
                Error::DbUpdateError
                    .into_msg()
                    .with_msg("查询用户分享码失败")
            })?;

        if result.is_some() {
            error!("用户分享码已存在");
            return Ok(true);
        }

        Ok(false)
    }
}

/// 权限相关
impl UserBaseService {
    /// 检查用户名称是否存在
    pub async fn check_username(&self, req: GetCheckUsernameReq) -> Result<(), ErrorMsg> {
        let result = self
            .user_base_dao
            .info_by_username(req.username)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;

        // 存在
        if let Some(_model) = result {
            error!("用户名称已存在");
            return Err(Error::DbDataExistError
                .into_msg()
                .with_msg("用户名称已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 获取用户信息个人信息
    pub async fn profile(&self, id: i32) -> Result<ProfileResp, ErrorMsg> {
        let user = self
            .user_base_dao
            .info(id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("用户信息不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("用户信息不存在")
            })?;

        let result = ProfileResp {
            id,
            username: user.username,
            gender: user.gender as i8,
            age: user.age,
            date_birth: user.date_birth,
            avatar: user.avatar,
        };
        Ok(result)
    }

    /// 通过用户信息ID获角色色列表
    pub async fn roles(&self, req: RolesReq) -> Result<(Vec<role::Model>, u64), ErrorMsg> {
        let (results, total) = self.user_base_dao.roles(req.user_id).await.map_err(|err| {
            error!("查询用户信息失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
        })?;

        Ok((results, total))
    }

    /// 获取用户信息
    pub async fn info_checked(&self, user_id: i32) -> Result<user_base::Model, ErrorMsg> {
        let user = self
            .user_base_dao
            .info(user_id)
            .await
            .map_err(|err| {
                error!("user_id: {user_id}, 查询用户信息失败, err: {err}",);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("user_id: {user_id}, 用户不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("用户不存在")
            })?;

        if !user.status {
            error!("user_id: {user_id}, 用户已被禁用");
            return Err(code::Error::LoginStatusDisabled
                .into_msg()
                .with_msg("用户已被禁用"));
        }

        Ok(user)
    }
}
