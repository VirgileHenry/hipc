/// The name of a workspace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceName(pub(crate) String);

impl std::ops::Deref for WorkspaceName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<String> for WorkspaceName {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for WorkspaceName {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
