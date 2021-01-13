// `PartialEq` and `Clone` is required
#[derive(PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    ADMIN,
    MANAGER,
}
