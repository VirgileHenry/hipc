#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SolitaryBlockedReason {
    Unknown,
    Notification,
    Lock,
    Workspace,
    Windowed,
    Dnd,
    Special,
    Alpha,
    Offset,
    Candidate,
    Opaque,
    Transform,
    Overlays,
    Float,
    Workspaces,
    Surfaces,
    Configerror,
    Fadeout,
    /// A reason added in a newer Hyprland version.
    #[serde(other)]
    Other,
}
