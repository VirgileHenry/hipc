/// The workspace reference embedded in a monitor.
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
pub struct WorkspaceIdAndName {
    pub id: crate::types::WorkspaceId,
    pub name: crate::types::WorkspaceName,
}
