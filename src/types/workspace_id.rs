/// The id of a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct WorkspaceId(pub(crate) i64);

impl WorkspaceId {
    /// Get the inner id value
    pub fn raw(&self) -> i64 {
        self.0
    }
}
