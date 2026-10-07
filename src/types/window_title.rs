/// The title of a window.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct WindowTitle(pub(crate) String);

impl std::ops::Deref for WindowTitle {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<String> for WindowTitle {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for WindowTitle {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
