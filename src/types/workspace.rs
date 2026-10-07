/// A workspace, as returned by `hyprctl -j workspace`
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: crate::types::WorkspaceId,
    pub name: crate::types::WorkspaceName,
    pub monitor: crate::types::MonitorName,
    #[serde(rename = "monitorID")]
    pub monitor_id: crate::types::MonitorId,
    pub windows: usize,
    #[serde(rename = "hasfullscreen")]
    pub has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    pub last_window: crate::types::WindowAddress,
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: crate::types::WindowTitle,
    #[serde(rename = "ispersistent")]
    pub is_persistent: bool,
    pub tiled_layout: crate::types::TiledLayout,
}
