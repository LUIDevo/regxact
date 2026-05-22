
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Allow {
    Exponential,
    MultiLine,
    DotAll,
    Wildcard,
}
