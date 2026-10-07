/// Sent as an array `[width, height]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}
