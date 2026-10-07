#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TearingBlockedReason {
    Unknown,
    NotTorn,
    User,
    Zoom,
    Support,
    Candidate,
    Window,
    HwCursor,
    /// A reason added in a newer Hyprland version.
    #[serde(other)]
    Other,
}
