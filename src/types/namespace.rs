/// The Namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct Namespace(pub(crate) String);

impl std::ops::Deref for Namespace {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
