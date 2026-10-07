/// The Name of a Submap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct SubmapName(pub(crate) String);

impl std::ops::Deref for SubmapName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<String> for SubmapName {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for SubmapName {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
