//! 登出

use std::sync::Arc;

use axum_context::Context;
use tower_sessions::Session;

use nject::injectable;

use code::{Error, ErrorMsg};
use user::{UserLoginLogDao, enums::user_login_log::LoginStatus};

use crate::{common::user_login_log::add_login_log, dto::login::BrowserInfo};

/// 服务层
#[injectable]
pub struct Logoutervice {
    #[inject(|x: UserLoginLogDao| Arc::new(x))]
    user_login_log_dao: Arc<UserLoginLogDao>,
}

impl Logoutervice {
    /// 登出
    pub async fn logout(
        &self,
        ctx: Context,

        browser_info: BrowserInfo,
        session: Session,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();
        let username = ctx.get_user_name();
        let session_id = session.id().ok_or(Error::SessionIdNotFound)?.to_string();

        // 添加登陆日志
        add_login_log(
            self.user_login_log_dao.clone(),
            user_id,
            username,
            browser_info,
            session_id.clone(),
            "登出",
            LoginStatus::Success,
        );

        // 删除session
        session
            .delete()
            .await
            .map_err(|err| Error::SessionIdDeleteError(err.to_string()))?;
        Ok(())
    }
}
