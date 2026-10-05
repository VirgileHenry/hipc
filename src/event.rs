/// All events emmited by Hyprland.
///
/// See [https://wiki.hypr.land/IPC/].
pub enum HyprlandEvent {
    /// Emitted on workspace change.
    ///
    /// Is emitted ONLY when a user requests a workspace change, and is not emitted on mouse movements (see focusedmon)
    Workspace {
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// Emitted on workspace change.
    ///
    /// Is emitted ONLY when a user requests a workspace change, and is not emitted on mouse movements (see focusedmon)
    WorkspaceV2 {
        /// WORKSPACEID
        id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// Emitted on the active monitor being changed.
    FocusedMonitor {
        /// MONNAME
        monitor: crate::types::MonitorName,
        /// WORKSPACENAME
        worskpace: crate::types::WorkspaceName,
    },
    /// Emitted on the active monitor being changed.
    FocusedMonitorV2 {
        /// MONNAME
        monitor: crate::types::MonitorName,
        /// WORKSPACENAME
        worskpace: crate::types::WorkspaceId,
    },
    /// Emitted on the active window being changed.
    ActiveWindow {
        /// WINDOWCLASS
        class: crate::types::WindowClass,
        /// WINDOWTITLE
        title: crate::types::WindowTitle,
    },
    /// Emitted on the active window being changed.
    ActiveWindowV2 {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when a fullscreen status of a window changes.
    Fullscreen {
        /// 0/1 (exit fullscreen / enter fullscreen)
        enter_fullscreen: bool,
    },
    /// emitted when a monitor is removed (disconnected)
    MonitorRemoved {
        /// MONITORNAME
        monitor: crate::types::MonitorName,
    },
    /// emitted when a monitor is removed (disconnected)
    MonitorRemovedV2 {
        /// MONITORID
        id: crate::types::MonitorId,
        /// MONITORNAME
        name: crate::types::MonitorName,
        /// MONITORDESCRIPTION
        description: String,
    },
    /// emitted when a monitor is added (connected)
    MonitorAdded {
        /// MONITORNAME
        name: crate::types::MonitorName,
    },
    /// emitted when a monitor is added (connected)
    MonitorAddedV2 {
        /// MONITORID
        id: crate::types::MonitorId,
        /// MONITORNAME
        name: crate::types::MonitorName,
        /// MONITORDESCRIPTION
        description: String,
    },
    /// emitted when a workspace is created
    CreateWorkspace {
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// emitted when a workspace is created
    CreateWorkspaceV2 {
        /// WORKSPACEID
        id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// emitted when a workspace is destroyed
    DestroyWorkspace {
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// emitted when a workspace is destroyed
    DestroyWorkspaceV2 {
        /// WORKSPACEID
        id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        name: crate::types::WorkspaceName,
    },
    /// emitted when a workspace is moved to a different monitor
    MoveWorkspace {
        /// WORKSPACENAME
        workspace: crate::types::WorkspaceName,
        /// MONNAME
        monitor: crate::types::MonitorName,
    },
    /// emitted when a workspace is moved to a different monitor
    MoveWorkspaceV2 {
        /// WORKSPACENAME
        workspace_id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        workspace_name: crate::types::WorkspaceName,
        /// MONNAME
        monitor: crate::types::MonitorName,
    },
    /// emitted when a workspace is renamed
    RenameWorkspace {
        /// WORKSPACEID
        id: crate::types::WorkspaceId,
        /// NEWNAME
        name: crate::types::WorkspaceName,
    },
    /// emitted when the special workspace opened in a monitor changes
    /// (closing results in an empty WORKSPACENAME)
    ActiveSpecial {
        /// WORKSPACENAME
        workspace: crate::types::WorkspaceName,
        /// MONNAME
        monitor: crate::types::MonitorName,
    },
    /// emitted when the special workspace opened in a monitor changes
    /// (closing results in empty WORKSPACEID and WORKSPACENAME values)
    ActiveSpecialV2 {
        /// WORKSPACEID
        workspace_id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        workspace_name: crate::types::WorkspaceName,
        /// MONNAME
        monitor: crate::types::MonitorName,
    },
    /// emitted on a layout change of the active keyboard
    ActiveLayout {
        /// KEYBOARDNAME
        keyboard: crate::types::KeyboardName,
        /// LAYOUTNAME
        layout: crate::types::LayoutName,
    },
    /// emitted when a window is opened
    OpenWindow {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// WORKSPACENAME
        workspace: crate::types::WorkspaceName,
        /// WINDOWCLASS
        class: crate::types::WindowClass,
        /// WINDOWTITLE
        title: crate::types::WindowTitle,
    },
    /// emitted when a window is closed
    CloseWindow {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when a window is killed (via hyprctl kill)
    Kill {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when a window is moved to a workspace
    MoveWindow {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// WORKSPACENAME
        workspace: crate::types::WorkspaceName,
    },
    /// emitted when a window is moved to a workspace
    MoveWindowV2 {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// WORKSPACEID
        workspace_id: crate::types::WorkspaceId,
        /// WORKSPACENAME
        workspace_name: crate::types::WorkspaceName,
    },
    /// emitted when a layerSurface is mapped
    OpenLayer {
        /// NAMESPACE
        namespace: crate::types::Namespace,
    },
    /// emitted when a layerSurface is unmapped
    CloseLayer {
        /// NAMESPACE
        namespace: crate::types::Namespace,
    },
    /// emitted when a keybind submap changes. Empty means default.
    Submap {
        /// SUBMAPNAME
        submap: crate::types::SubmapName,
    },
    /// emitted when a window changes its floating mode. FLOATING is either 0 or 1.
    ChangeFloatingMode {
        /// WINDOWADDRESS
        window: crate::types::WindowAddress,
        /// FLOATING
        floating: bool,
    },
    /// emitted when a window requests an urgent state
    Urgent {
        /// WINDOWADDRESS
        window: crate::types::WindowAddress,
    },
    /// emitted when a screencopy state of a client changes.
    /// Keep in mind there might be multiple separate clients. State is 0/1, owner is monitor/window/region
    ScreenCast {
        /// STATE
        state: bool,
        /// OWNER
        owner: crate::types::Owner,
    },
    /// emitted when a screencopy state of a client changes.
    /// Keep in mind there might be multiple separate clients. State is 0/1, owner is monitor/window/region,
    /// name is the identifier of the shared target (monitor name or window title)
    ScreenCastV2 {
        /// STATE
        state: bool,
        /// OWNER
        owner: crate::types::Owner,
        /// NAME
        name: String,
    },
    /// emitted when a window title changes.
    WindowTitle {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when a window title changes.
    WindowTitleV2 {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// WINDOWTITLE
        title: crate::types::WindowTitle,
    },
    /// emitted when togglegroup command is used.
    ///
    /// state,handle where the state is a toggle status and the handle is one or more window addresses separated by a comma
    /// e.g. 0,64cea2525760,64cea2522380 where 0 means that a group has been destroyed and the rest informs which windows were part of it
    ToggleGroup {
        /// 0/1
        state: bool,
        /// WINDOWADDRESS(ES)
        addresses: Vec<crate::types::WindowAddress>,
    },
    /// emitted when the window is merged into a group. returns the address of a merged window
    MoveIntoGroup {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when the window is removed from a group. returns the address of a removed window
    MoveOutOfGroup {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
    /// emitted when ignoregrouplock is toggled.
    IgnoreGroupLock {
        /// 0/1
        enabled: bool,
    },
    /// emitted when lockgroups is toggled.
    LockGroups {
        /// 0/1
        locked: bool,
    },
    /// emitted when the config is done reloading
    ConfigReloaded {},
    /// emitted when a window is pinned or unpinned
    Pin {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// PINSTATE
        pinned: bool,
    },
    /// emitted when an external taskbar-like app requests a window to be minimized
    Minimized {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
        /// 0/1
        minimized: bool,
    },
    /// emitted when an app requests to ring the system bell via xdg-system-bell-v1. Window address parameter may be empty.
    Bell {
        /// WINDOWADDRESS
        address: crate::types::WindowAddress,
    },
}
