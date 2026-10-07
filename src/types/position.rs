/// Sent as an array `[x, y]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
