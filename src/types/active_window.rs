/// A window, as returned by `hyprctl -j activewindow` (and each element of `hyprctl -j clients`).
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveWindow {
    pub address: crate::types::WindowAddress,
    pub mapped: bool,
    pub hidden: bool,
    /// Mapped, accepts input and isn't fully transparent.
    pub visible: bool,
    pub accepts_input: bool,
    /// Position in the global layout, in logical pixels.
    pub at: crate::types::Position,
    /// In logical pixels.
    pub size: crate::types::Size,
    pub workspace: crate::types::WorkspaceIdAndName,
    pub floating: bool,
    /// `-1` if the window isn't on any monitor.
    pub monitor: crate::types::MonitorId,
    /// Current class, can change over the window's lifetime.
    pub class: crate::types::WindowClass,
    pub title: crate::types::WindowTitle,
    /// Class when the window was mapped.
    pub initial_class: crate::types::WindowClass,
    /// Title when the window was mapped.
    pub initial_title: crate::types::WindowTitle,
    pub pid: i32,
    pub xwayland: bool,
    pub pinned: bool,
    /// The fullscreen state Hyprland applies.
    pub fullscreen: crate::types::FullscreenMode,
    /// The fullscreen state the client thinks it's in.
    pub fullscreen_client: crate::types::FullscreenMode,
    pub over_fullscreen: bool,
    /// Every window of the group this window is in, itself included. Empty if not grouped.
    pub grouped: Vec<crate::types::WindowAddress>,
    pub tags: Vec<String>,
    /// The window this one swallowed, if any.
    #[serde(deserialize_with = "deserialize_null_address")]
    pub swallowing: Option<crate::types::WindowAddress>,
    /// 0 is the currently focused window, 1 the previous one, and so on. `-1` if never focused.
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: i32,
    pub inhibiting_idle: bool,
    /// Empty if unset.
    pub xdg_tag: String,
    /// Empty if unset.
    pub xdg_description: String,
    pub content_type: crate::types::ContentType,
    /// Sent as hex without a `0x` prefix.
    #[serde(deserialize_with = "deserialize_hex")]
    pub stable_id: u64,
}

/// `"0x0"` means no window.
fn deserialize_null_address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<crate::types::WindowAddress>, D::Error> {
    use serde::Deserialize;
    let s = <&str>::deserialize(d)?;
    if s == "0x0" {
        return Ok(None);
    }
    s.parse().map(Some).map_err(serde::de::Error::custom)
}

fn deserialize_hex<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    use serde::Deserialize;
    let s = <&str>::deserialize(d)?;
    u64::from_str_radix(s, 16).map_err(serde::de::Error::custom)
}
