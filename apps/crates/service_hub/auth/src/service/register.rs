//! 注册
use crate::{
    common::captcha::check_captcha, dao::register::RegisterDao, dto::register::RegisterReq,
};

use system::ImageCaptchaDao;
use user::{BlockchainWalletDao, EmailDao, PhoneDao, UserBaseDao};

use code::{Error, ErrorMsg};
use entity::user::user_base;
use utils::crypto::sha2_256;

use nject::injectable;
use tracing::error;

/// 服务层
#[injectable]
pub struct RegisterService {
    user_dao: UserBaseDao,
    email_dao: EmailDao,
    phone_dao: PhoneDao,
    blockchain_wallet_dao: BlockchainWalletDao,
    register_dao: RegisterDao,
    captcha_dao: ImageCaptchaDao,
}

impl RegisterService {
    /// 根据不同的注册类型进行注册用户
    pub async fn register(&self, req: RegisterReq) -> Result<(), ErrorMsg> {
        // 参数校验, 防止被空刷验证码
        match req.register_type {
            user_base::enums::UserType::Base => {
                if req.phone.is_none() {
                    error!("请输入用户名");
                    return Err(Error::InvalidParameter("请输入用户名".to_string()).into_msg());
                }
            }
            user_base::enums::UserType::Phone => {
                if req.phone.is_none() {
                    error!("请输入手机号码");
                    return Err(Error::InvalidParameter("请输入手机号码".to_string()).into_msg());
                }
            }
            user_base::enums::UserType::Email => {
                if req.email.is_none() {
                    error!("请输入邮箱");
                    return Err(Error::InvalidParameter("请输入邮箱".to_string()).into_msg());
                }
            }
            user_base::enums::UserType::BlockchainWallet => {
                if req.phone.is_none() {
                    error!("请输入钱包地址");
                    return Err(Error::InvalidParameter("请输入钱包地址".to_string()).into_msg());
                }
            }
        }

        // 检测验证码
        check_captcha(
            &self.captcha_dao,
            req.captcha_id.clone(),
            req.captcha.clone(),
        )
        .await?;

        // 检查用户名, 查看用户名是否已注册
        self.check_username(req.username.clone()).await?;

        // 根据不同注册类型进行注册检查
        match req.register_type {
            user_base::enums::UserType::Base => self.check_username(req.username.clone()).await?,
            user_base::enums::UserType::Phone => self.check_phone(req.clone()).await?,
            user_base::enums::UserType::Email => self.check_email(req.clone()).await?,
            user_base::enums::UserType::BlockchainWallet => {
                self.check_blockchain_wallet(req.clone()).await?
            }
        };

        let mut data = req.clone();

        // 密码加密
        data.password = sha2_256(&data.password);

        // 添加用户
        let _result = self.register_dao.add_user(data).await.map_err(|err| {
            error!("注册用户失败, err: {:#?}", err);
            Error::DbAddError.into_msg().with_msg("注册用户失败")
        })?;

        Ok(())
    }

    /// 检查用户名, 查看用户名是否已注册
    async fn check_username(&self, username: String) -> Result<(), ErrorMsg> {
        let result = self
            .user_dao
            .info_by_username(username)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
        if result.is_some() {
            return Err(Error::UserAddError.into_msg().with_msg("用户名已存在"));
        }
        Ok(())
    }

    /// 检查手机号码
    async fn check_phone(&self, req: RegisterReq) -> Result<(), ErrorMsg> {
        let phone = match req.phone.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter(
                    "请求参数错误, phone 不能为空".to_string(),
                )
                .into_msg())
            }
        };

        // TODO 检测手机验证码, 待接入第三方服务

        // 检测是否已注册用户
        let phone = self.phone_dao.info_by_phone(phone).await.map_err(|err| {
            error!("查询用户信息失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
        })?;
        if phone.is_some() {
            {
                error!("该手机号码已注册");
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("该手机号码已注册"));
            };
        }

        Ok(())
    }

    /// 检查邮箱
    async fn check_email(&self, data: RegisterReq) -> Result<(), ErrorMsg> {
        let email = match data.email.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("请求参数错误, email 不能为空"))
            }
        };

        // 检测是否已注册邮箱
        let user = self.email_dao.info_by_email(email).await.map_err(|err| {
            error!("查询用户信息失败, err: {:#?}", err);
            Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
        })?;
        if user.is_some() {
            {
                error!("该邮箱已注册");
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("该邮箱已注册"));
            };
        }

        // TODO 邮箱验证, 发送链接点击后确认, 确认完毕后启用用户

        Ok(())
    }

    /// 检查区块链钱包
    async fn check_blockchain_wallet(&self, req: RegisterReq) -> Result<(), ErrorMsg> {
        let blockchain_wallet = match req.blockchain_wallet.clone() {
            Some(v) => v,
            None => {
                return Err(code::Error::InvalidParameter(
                    "请求参数错误, blockchain_wallet 不能为空".to_string(),
                )
                .into_msg())
            }
        };

        // 检测是否已注册用户
        let blockchain_wallet = self
            .blockchain_wallet_dao
            .info_by_wallet_address(blockchain_wallet)
            .await
            .map_err(|err| {
                error!("查询用户信息失败, err: {:#?}", err);
                Error::DbQueryError.into_msg().with_msg("查询用户信息失败")
            })?;
        if blockchain_wallet.is_some() {
            {
                error!("该钱包已注册");
                return Err(code::Error::DbDataExistError
                    .into_msg()
                    .with_msg("该钱包已注册"));
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password() {
        let password = sha2_256("123456");
        assert_eq!(
            password,
            "da023f7090dd831097f8a534475b1c4fba2a9a6419968e52be7459e2533ac819"
        );
    }
}
