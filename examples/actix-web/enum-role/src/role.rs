// `Eq` and `Hash` is required
#[derive(Eq, Hash)]
pub enum Role {
    Admin,
    Manager,
}
