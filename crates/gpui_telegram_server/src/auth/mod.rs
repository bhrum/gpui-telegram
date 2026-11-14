pub mod passkey_service;
pub mod session_manager;

pub use crate::server::AuthBearer;
pub use passkey_service::{PasskeyChallenge, PasskeyService, PasskeyVerifyRequest};
pub use session_manager::{
    hash_password, verify_password, AuthError, SessionManager, UserRecord, MAX_PASSWORD_BYTES,
    MAX_USERNAME_CHARS,
};
