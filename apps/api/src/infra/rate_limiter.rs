use axum::http::{Request, StatusCode};
use tower_governor::{key_extractor::KeyExtractor, GovernorError};

use super::auth::AuthUser;

impl KeyExtractor for AuthUser {
    type Key = String;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
        req.headers()
            .get("Authorization")
            .and_then(|token| token.to_str().ok())
            .and_then(|token| token.strip_prefix("Bearer "))
            .and_then(|token| Some(token.trim().to_owned()))
            .ok_or(GovernorError::Other {
                code: StatusCode::UNAUTHORIZED,
                msg: Some("You don't have permission to access".to_string()),
                headers: None,
            })
    }

    fn key_name(&self, key: &Self::Key) -> Option<String> {
        Some(format!("{}", key))
    }

    fn name(&self) -> &'static str {
        "AuthUser"
    }
}
