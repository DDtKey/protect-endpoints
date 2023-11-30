// `Eq` and `Hash` is required
#[derive(Eq, PartialEq, Hash)]
pub enum Role {
    Admin,
    Manager,
}
