use poem::error::{Forbidden, Unauthorized};

#[derive(Debug, thiserror::Error)]
pub enum AccessError {
    #[error("Unauthorized request")]
    UnauthorizedRequest,
    #[error("Forbidden request")]
    ForbiddenRequest,
}

impl From<AccessError> for poem::Error {
    fn from(value: AccessError) -> Self {
        match value {
            e @ AccessError::UnauthorizedRequest => Unauthorized(e),
            e @ AccessError::ForbiddenRequest => Forbidden(e),
        }
    }
}
