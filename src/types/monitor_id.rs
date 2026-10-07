/// The id of a monitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct MonitorId(pub(crate) i64);

impl MonitorId {
    /// Get the inner id value
    pub fn raw(&self) -> i64 {
        self.0
    }
}
