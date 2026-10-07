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

impl std::borrow::Borrow<String> for MonitorName {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl std::borrow::BorrowMut<String> for MonitorName {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
