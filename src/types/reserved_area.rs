/// Area reserved on each edge (bars, etc.), in logical pixels.
///
/// Sent as an array in the order `[left, top, right, bottom]`.
#[derive(Debug, Clone, Copy)]
#[derive(serde::Deserialize)]
pub struct ReservedArea {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
