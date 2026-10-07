#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectScanoutBlockedReason {
    Unknown,
    User,
    Windowed,
    Content,
    Mirror,
    Record,
    Sw,
    Candidate,
    Surface,
    Transform,
    Dma,
    Failed,
    Cm,
    /// A reason added in a newer Hyprland version.
    #[serde(other)]
    Other,
}
