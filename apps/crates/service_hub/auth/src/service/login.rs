//! 登陆

use std::sync::Arc;

use crate::{
    common::captcha::check_captcha,
    dto::login::{BrowserInfo, LoginReq, LoginResp},
};

use code::{Error, ErrorMsg};
use entity::{user::user_base, user::user_login_log};
use system::ImageCaptchaDao;
use tower_sessions::Session;
use user::{EmailDao, PhoneDao, UserBaseDao, UserLoginLogDao};
use utils::browser::parse_user_agent_async;

use nject::injectable;
use sea_orm::Set;
use tracing::error;

/// 服务层
#[injectable]
pub struct LoginService {
    user_dao: UserBaseDao,
    #[inject(|x: UserLoginLogDao| Arc::new(x))]
    user_login_dao: Arc<UserLoginLogDao>,
    email_dao: EmailDao,
    phone_dao: PhoneDao,
    captcha_dao: ImageCaptchaDao,
}

impl LoginService {
    /// 登陆
    pub async fn login(
        &self,
        req: LoginReq,
        browser_info: BrowserInfo,
        session: Session,
    ) -> Result<LoginResp, ErrorMsg> {
        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            req.captcha_id.clone(),
            req.captcha.clone(),
        )
        .await
        .inspect_err(|err| {
            error!("验证码校验失败, err: {err}");
        })?;

        // 检测手机号码或邮件用户是否存在
        let user = self.get_user(req.clone()).await?;
        // 检查用户是否被禁用
        if !user.status {
            error!("{} 用户已被禁用", user.id);
            self.add_login_log(
                user.clone(),
                browser_info,
                "".to_string(),
                Some("用户已被禁用".to_owned()),
                user_login_log::enums::LoginStatus::Failed,
            );
            return Err(Error::LoginUserDisableError
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        // 检测密码
        if user.password != req.password {
            error!("{} 账号或密码错误", user.id);
            self.add_login_log(
                user.clone(),
                browser_info,
                "".to_string(),
                Some("账号或密码错误".to_owned()),
                user_login_log::enums::LoginStatus::Failed,
            );
            return Err(Error::LoginPasswordError
                .into_msg()
                .with_msg("账号或密码错误"));
        }

        session
            .insert("user_id", user.id.clone())
            .await
            .map_err(|err| Error::SessionIdInsertError(err.to_string()))?;
        session
            .insert("username", user.username.clone())
            .await
            .map_err(|err| Error::SessionIdInsertError(err.to_string()))?;
        let session_id = session.id().ok_or(Error::SessionIdNotFound)?.0.to_string();

        // 添加登陆日志
        self.add_login_log(
            user.clone(),
            browser_info,
            session_id.clone(),
            None,
            user_login_log::enums::LoginStatus::Success,
        );

        // 返回Token
        Ok(LoginResp { user_id: user.id })
    }

    /// 获取用户信息
    async fn get_user(&self, data: LoginReq) -> Result<user_base::Model, ErrorMsg> {
        let user_id = match data.user_type {
            user_base::enums::UserType::Phone => self.get_user_phone(data).await?,
            user_base::enums::UserType::Email => self.get_user_email(data).await?,
        };

        // 查询用户
        let result = self
            .user_dao
            .info(user_id)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("该用户不存在")
            })?;

        Ok(result)
    }

    /// 获取用户手机号
    async fn get_user_phone(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let phone = match data.phone.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter(
                    "请求参数错误, phone 不能为空".to_string(),
                )
                .into_msg())
            }
        };

        let user = self
            .phone_dao
            .info_by_phone(phone)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户手机号不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("该用户手机号不存在")
            })?;

        Ok(user.user_id)
    }

    /// 获取用户邮箱
    async fn get_user_email(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let email = match data.email.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter(
                    "请求参数错误, email 不能为空".to_string(),
                )
                .into_msg())
            }
        };

        let user = self
            .email_dao
            .info_by_email(email)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户邮箱不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("该用户邮箱不存在")
            })?;

        Ok(user.user_id)
    }

    /// 添加登陆日志
    fn add_login_log(
        &self,
        user: user_base::Model,
        browser_info: BrowserInfo,
        session_id: String,
        desc: Option<String>,
        login_status: user_login_log::enums::LoginStatus,
    ) {
        let user_login_dao = self.user_login_dao.clone();

        tokio::task::spawn(async move {
            let (device, system, browser) =
                match parse_user_agent_async(browser_info.user_agent.clone()).await {
                    Ok(v) => v,
                    Err(err) => {
                        error!("User-Agent解析错误, err: {:#?}", err);
                        return;
                    }
                };

            let data = user_login_log::ActiveModel {
                user_id: Set(user.id),
                username: Set(user.username),
                session_id: Set(session_id),
                remote_addr: Set(browser_info.remote_addr),
                user_agent: Set(browser_info.user_agent),
                login_status: Set(login_status as i8),
                device: Set(Some(device)),
                system: Set(Some(system)),
                browser: Set(Some(browser)),
                desc: Set(desc),
                ..Default::default()
            };

            let result = user_login_dao.create(data).await.map_err(|err| {
                error!("添加登陆日志失败, err: {:#?}", err);
                code::Error::DbAddError
                    .into_msg()
                    .with_msg("添加登陆日志失败")
            });
            if let Err(err) = result {
                error!("添加登陆日志失败, err: {:#?}", err);
            }
        });
    }
}
