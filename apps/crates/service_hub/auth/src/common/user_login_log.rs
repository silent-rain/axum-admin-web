//! 登录日志

use std::sync::Arc;

use err_code::Error;
use log::error;
use sea_orm::Set;

use user::{UserLoginLogDao, entity::user_login_log, enums::user_login_log::LoginStatus};
use utils::browser::parse_user_agent_async;

use crate::dto::login::BrowserInfo;

/// 添加登录日志
pub fn add_login_log(
    user_login_log_dao: Arc<UserLoginLogDao>,
    user_id: i32,
    username: String,
    browser_info: BrowserInfo,
    session_id: String,
    desc: &str,
    login_status: LoginStatus,
) {
    let user_login_dao = user_login_log_dao.clone();
    let desc = desc.to_string();

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
            user_id: Set(user_id),
            username: Set(username),
            session_id: Set(session_id.clone()),
            remote_addr: Set(browser_info.remote_addr),
            user_agent: Set(browser_info.user_agent),
            login_status: Set(login_status as i8),
            device: Set(Some(device)),
            system: Set(Some(system)),
            browser: Set(Some(browser)),
            desc: Set(Some(desc)),
            ..Default::default()
        };

        let result = user_login_dao.create(data).await.map_err(|err| {
            error!(
                "添加登陆日志失败, session_id: {:#?}, err: {:#?}",
                session_id, err
            );
            Error::DbAddError.into_err_with_msg("添加登陆日志失败")
        });
        if let Err(err) = result {
            error!("添加登陆日志失败, err: {:#?}", err);
        }
    });
}
