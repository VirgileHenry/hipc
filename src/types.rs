//! Common types used by Hyprland.

/// The name of a workspace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceName(pub(crate) String);

impl std::ops::Deref for WorkspaceName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The id of a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkspaceId(pub(crate) i64);

impl WorkspaceId {
    /// Get the inner id value
    pub fn raw(&self) -> i64 {
        self.0
    }
}

/// The name of a monitor.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorName(pub(crate) String);

impl std::ops::Deref for MonitorName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The id of a monitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorId(pub(crate) i64);

impl MonitorId {
    /// Get the inner id value
    pub fn raw(&self) -> i64 {
        self.0
    }
}

/// The title of a window.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowTitle(pub(crate) String);

impl std::ops::Deref for WindowTitle {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The title of a window.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowClass(pub(crate) String);

impl std::ops::Deref for WindowClass {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The address of a window, a unique hex handle to identify it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowAddress(u64);

impl std::str::FromStr for WindowAddress {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("0x").unwrap_or(s);
        u64::from_str_radix(hex, 16).map(Self)
    }
}

impl std::fmt::Display for WindowAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

/// The name of a keyboard.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyboardName(pub(crate) String);

impl std::ops::Deref for KeyboardName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The name of a layout.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutName(pub(crate) String);

impl std::ops::Deref for LayoutName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The Namespace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(pub(crate) String);

impl std::ops::Deref for Namespace {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The Name of a Submap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmapName(pub(crate) String);

impl std::ops::Deref for SubmapName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Owner of a screen cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Owner {
    Monitor,
    Window,
    Region,
}

impl std::str::FromStr for Owner {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monitor" => Ok(Self::Monitor),
            "window" => Ok(Self::Window),
            "region" => Ok(Self::Region),
            _ => Err(()),
        }
    }
}
