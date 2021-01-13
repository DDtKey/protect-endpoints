use poem::error::ResponseError;
use poem::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AccessError {
    #[error("Unauthorized request")]
    UnauthorizedRequest,
    #[error("Forbidden request")]
    ForbiddenRequest,
}

impl ResponseError for AccessError {
    fn status(&self) -> StatusCode {
        match self {
            AccessError::UnauthorizedRequest => StatusCode::UNAUTHORIZED,
            AccessError::ForbiddenRequest => StatusCode::FORBIDDEN,
        }
    }
}
