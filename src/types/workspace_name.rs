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
