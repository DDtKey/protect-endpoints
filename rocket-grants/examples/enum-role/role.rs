// `Eq` and `Hash` is required
#[derive(PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Manager,
}
