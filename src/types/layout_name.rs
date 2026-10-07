/// The name of a layout.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct LayoutName(pub(crate) String);

impl std::ops::Deref for LayoutName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<String> for LayoutName {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for LayoutName {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
