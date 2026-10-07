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

impl std::borrow::Borrow<String> for KeyboardName {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for KeyboardName {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
