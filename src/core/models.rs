#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TmdbId(pub String);

impl From<String> for TmdbId {
    fn from(id: String) -> Self {
        TmdbId(id)
    }
}
