/// The name of a keyboard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct KeyboardName(pub(crate) String);

impl std::ops::Deref for KeyboardName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
