/// The title of a window.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct WindowClass(pub(crate) String);

impl std::ops::Deref for WindowClass {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
