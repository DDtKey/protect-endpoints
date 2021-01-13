use crate::authorities::AuthDetails;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;

pub trait AttachAuthorities {
    fn attach(&self, authorities: Vec<String>);
}

impl AttachAuthorities for ServiceRequest {
    fn attach(&self, authorities: Vec<String>) {
        self.extensions_mut().insert(AuthDetails::new(authorities));
    }
}
