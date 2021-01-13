pub mod authorities;
mod middleware;

pub use middleware::GrantsMiddleware;

pub mod proc_macro {
    pub use actix_grants_proc_macro::*;
}
