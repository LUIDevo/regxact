
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Allow {
    Exponential,
    DotAll,
    Wildcard,
}
