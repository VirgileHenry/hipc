/// A monitor, as returned by `hyprctl monitors`.
#[derive(Debug, Clone)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: crate::types::MonitorId,
    pub name: crate::types::MonitorName,
    pub description: String,
    pub make: String,
    pub model: String,
    /// Empty if the monitor doesn't report one.
    pub serial: String,
    /// In pixels.
    pub width: u32,
    /// In pixels.
    pub height: u32,
    /// In millimeters, 0 if unknown.
    pub physical_width: u32,
    /// In millimeters, 0 if unknown.
    pub physical_height: u32,
    /// In Hz.
    pub refresh_rate: f64,
    /// Position in the global layout, in logical pixels. Can be negative.
    pub x: i32,
    /// Position in the global layout, in logical pixels. Can be negative.
    pub y: i32,
    pub active_workspace: crate::types::WorkspaceIdAndName,
    /// `id: 0` and an empty name when no special workspace is open.
    pub special_workspace: crate::types::WorkspaceIdAndName,
    pub reserved: crate::types::ReservedArea,
    pub scale: f64,
    /// `wl_output` transform, 0 to 7.
    pub transform: u8,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool,
    /// The window currently in solitary mode, if any.
    #[serde(deserialize_with = "deserialize_null_address")]
    pub solitary: Option<crate::types::WindowAddress>,
    /// `None` when solitary mode isn't blocked.
    pub solitary_blocked_by: Option<Vec<crate::types::SolitaryBlockedReason>>,
    pub actively_tearing: bool,
    /// `None` when tearing isn't blocked.
    pub tearing_blocked_by: Option<Vec<crate::types::TearingBlockedReason>>,
    /// Hex pointer to the internal surface being directly scanned out, `"0"` if none.
    pub direct_scanout_to: String,
    /// `None` when direct scanout isn't blocked.
    pub direct_scanout_blocked_by: Option<Vec<crate::types::DirectScanoutBlockedReason>>,
    pub disabled: bool,
    /// DRM format, one of `XRGB8888`, `XBGR8888`, `XRGB2101010`, `XBGR2101010` or `Invalid`.
    pub current_format: String,
    #[serde(deserialize_with = "deserialize_mirror_of")]
    pub mirror_of: Option<crate::types::MonitorId>,
    /// Formatted as `WIDTHxHEIGHT@RATEHz`, e.g. `1920x1080@60.00Hz`.
    pub available_modes: Vec<String>,
    pub color_management_preset: String,
    pub sdr_brightness: f64,
    pub sdr_saturation: f64,
    pub sdr_min_luminance: f64,
    pub sdr_max_luminance: f64,
    pub hardware_cursors_in_use: bool,
}

/// `"0"` means no window.
fn deserialize_null_address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<crate::types::WindowAddress>, D::Error> {
    use serde::Deserialize;
    let s = <&str>::deserialize(d)?;
    if s == "0" {
        return Ok(None);
    }
    s.parse().map(Some).map_err(serde::de::Error::custom)
}

/// `"none"` or the mirrored monitor's id as a string.
fn deserialize_mirror_of<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<crate::types::MonitorId>, D::Error> {
    use serde::Deserialize;
    let s = <&str>::deserialize(d)?;
    if s == "none" {
        return Ok(None);
    }
    s.parse()
        .map(|id| Some(crate::types::MonitorId(id)))
        .map_err(serde::de::Error::custom)
}
