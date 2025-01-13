//! axum session
mod dao;
mod session_store;

use database::PoolTrait;
pub use session_store::DbStore;

use std::sync::Arc;

use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};

/// session layer
pub fn session_layer(db: Arc<dyn PoolTrait>) -> SessionManagerLayer<DbStore> {
    let session_store = DbStore::new(db);

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)))
}
