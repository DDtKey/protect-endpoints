// `Eq` and `Hash` is required
#[derive(Eq, PartialEq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    ADMIN,
    MANAGER,
}
