/// The `wp_content_type_v1` hint set by the client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    None,
    Photo,
    Video,
    Game,
}
