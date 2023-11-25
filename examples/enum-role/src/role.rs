// `PartialEq` and `Clone` is required
#[derive(PartialEq, Clone)]
pub enum Role {
    Admin,
    Manager,
}
