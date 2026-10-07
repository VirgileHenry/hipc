#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TiledLayout {
    Dwindle,
    Master,
    Scrolling,
    Monocle,
    #[serde(untagged)]
    Other(String),
}
