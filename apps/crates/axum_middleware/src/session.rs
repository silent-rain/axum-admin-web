//! session layer

use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

/// session layer
pub fn session_layer() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();

    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)))
}
