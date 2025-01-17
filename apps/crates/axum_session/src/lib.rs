//! axum session
use std::sync::Arc;

mod dao;
mod session_store;

use database::PoolTrait;
pub use session_store::DbStore;

pub use tower_sessions::Session;
use tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};

pub static SESSION_ID: &str = "session-id";

/// session layer
pub fn session_layer(db: Arc<dyn PoolTrait>) -> SessionManagerLayer<DbStore> {
    let session_store = DbStore::new(db);

    SessionManagerLayer::new(session_store)
        .with_name(SESSION_ID)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
}
