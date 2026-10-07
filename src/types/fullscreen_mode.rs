#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
#[serde(try_from = "u8")]
pub enum FullscreenMode {
    None,
    Maximized,
    Fullscreen,
    MaximizedAndFullscreen,
}

impl TryFrom<u8> for FullscreenMode {
    type Error = String;

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(Self::None),
            1 => Ok(Self::Maximized),
            2 => Ok(Self::Fullscreen),
            3 => Ok(Self::MaximizedAndFullscreen),
            other => Err(format!("Unknown fullscreen mode: {other}")),
        }
    }
}
