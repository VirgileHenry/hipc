/// The name of a monitor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct MonitorName(pub(crate) String);

impl std::ops::Deref for MonitorName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
