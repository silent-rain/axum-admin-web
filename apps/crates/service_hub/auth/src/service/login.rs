//! 登陆

use std::sync::Arc;

use nject::injectable;
use tracing::error;

use code::{Error, ErrorMsg};
use entity::{user::user_base, user::user_login_log};
use system::ImageCaptchaDao;
use tower_sessions::Session;
use user::{BlockchainWalletDao, EmailDao, PhoneDao, UserBaseDao, UserLoginLogDao};

use crate::{
    common::captcha::check_captcha,
    common::user_login_log::add_login_log,
    dto::login::{BrowserInfo, LoginReq, LoginResp},
};

/// 服务层
#[injectable]
pub struct LoginService {
    user_dao: UserBaseDao,
    email_dao: EmailDao,
    phone_dao: PhoneDao,
    blockchain_wallet_dao: BlockchainWalletDao,
    captcha_dao: ImageCaptchaDao,
    #[inject(|x: UserLoginLogDao| Arc::new(x))]
    user_login_log_dao: Arc<UserLoginLogDao>,
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

        let session_id = session.id().ok_or(Error::SessionIdNotFound)?.to_string();

        // 检测手机号码或邮件用户是否存在
        let user = self.get_user(req.clone()).await?;
        // 检查用户是否被禁用
        if !user.status {
            error!("{} 用户已被禁用", user.id);
            add_login_log(
                self.user_login_log_dao.clone(),
                user.id,
                user.username,
                browser_info,
                session_id.clone(),
                "用户已被禁用",
                user_login_log::enums::LoginStatus::Failed,
            );
            return Err(Error::LoginUserDisableError
                .into_msg()
                .with_msg("用户已被禁用"));
        }
        // 检测密码
        if user.password != req.password {
            error!("{} 账号或密码错误", user.id);
            add_login_log(
                self.user_login_log_dao.clone(),
                user.id,
                user.username,
                browser_info,
                session_id.clone(),
                "账号或密码错误",
                user_login_log::enums::LoginStatus::Failed,
            );

            return Err(Error::LoginPasswordError
                .into_msg()
                .with_msg("账号或密码错误"));
        }

        session
            .insert("user_id", user.id)
            .await
            .map_err(|err| Error::SessionIdInsertError(err.to_string()))?;
        session
            .insert("username", user.username.clone())
            .await
            .map_err(|err| Error::SessionIdInsertError(err.to_string()))?;

        // 添加登陆日志
        add_login_log(
            self.user_login_log_dao.clone(),
            user.id,
            user.username,
            browser_info,
            session_id.clone(),
            "登录成功",
            user_login_log::enums::LoginStatus::Success,
        );

        // 返回Token
        Ok(LoginResp { user_id: user.id })
    }

    /// 获取用户信息
    async fn get_user(&self, data: LoginReq) -> Result<user_base::Model, ErrorMsg> {
        let user_id = match data.user_type {
            user_base::enums::UserType::Base => self.get_user_base(data).await?,
            user_base::enums::UserType::Phone => self.get_user_phone(data).await?,
            user_base::enums::UserType::Email => self.get_user_email(data).await?,
            user_base::enums::UserType::BlockchainWallet => {
                self.get_user_blockchain_wallet(data).await?
            }
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

    /// 获取用户名用户
    async fn get_user_base(&self, req: LoginReq) -> Result<i32, ErrorMsg> {
        let username = match req.username.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter(
                    "请求参数错误, 用户名或密码 不能为空".to_string(),
                )
                .into_msg())
            }
        };

        let user = self
            .user_dao
            .info_by_username(username)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该用户名或密码不存在");
                Error::DbQueryEmptyError
                    .into_msg()
                    .with_msg("该用户名或密码不存在")
            })?;

        Ok(user.id)
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

    /// 获取区块链钱包
    async fn get_user_blockchain_wallet(&self, data: LoginReq) -> Result<i32, ErrorMsg> {
        let blockchain_wallet = match data.blockchain_wallet.clone() {
            Some(v) => v,
            None => {
                return Err(
                    code::Error::InvalidParameter("请求参数错误, 钱包不能为空".to_string())
                        .into_msg(),
                )
            }
        };

        let user = self
            .blockchain_wallet_dao
            .info_by_wallet_address(blockchain_wallet)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?
            .ok_or_else(|| {
                error!("该钱包不存在");
                Error::DbQueryEmptyError.into_msg().with_msg("该钱包不存在")
            })?;

        Ok(user.user_id)
    }
}
