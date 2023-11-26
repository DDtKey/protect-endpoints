// `PartialEq` and `Clone` is required
#[derive(PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Manager,
}
