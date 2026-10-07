//! Implementation of std::str::FromStr for HyprlandEvent.
//!
//! This is done in it's own module to keep the event with only the data and not bloat the file.

impl std::str::FromStr for crate::HyprlandEvent {
    type Err = InvalidEvent;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (event_kind, data) = s
            .split_once(">>")
            .ok_or(InvalidEvent::MalformedEvent { event: s.to_string() })?;
        /* Remove the ending newline */
        let data = data.trim_end();

        let malformed_data = || InvalidEvent::MalformedData {
            for_event: event_kind.to_string(),
            data: data.to_string(),
        };

        match event_kind {
            "workspace" => Ok(Self::Workspace {
                name: crate::types::WorkspaceName(data.to_string()),
            }),
            "workspacev2" => {
                let mut split = data.split(',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::WorkspaceV2 {
                    id: crate::types::WorkspaceId(id),
                    name: crate::types::WorkspaceName(name.to_string()),
                })
            }
            "focusedmon" => {
                let mut split = data.split(',');

                let monitor = split.next().ok_or_else(malformed_data)?;
                let workspace = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::FocusedMonitor {
                    monitor: crate::types::MonitorName(monitor.to_string()),
                    workspace: crate::types::WorkspaceName(workspace.to_string()),
                })
            }
            "focusedmonv2" => {
                let mut split = data.split(',');

                let monitor = split.next().ok_or_else(malformed_data)?;
                let workspace = split.next().ok_or_else(malformed_data)?;
                let workspace = workspace.parse().map_err(|_| malformed_data())?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::FocusedMonitorV2 {
                    monitor: crate::types::MonitorName(monitor.to_string()),
                    workspace: crate::types::WorkspaceId(workspace),
                })
            }
            "activewindow" => {
                // The title is last and may contain commas.
                let mut split = data.splitn(2, ',');

                let class = split.next().ok_or_else(malformed_data)?;
                let title = split.next().ok_or_else(malformed_data)?;

                Ok(Self::ActiveWindow {
                    class: crate::types::WindowClass(class.to_string()),
                    title: crate::types::WindowTitle(title.to_string()),
                })
            }
            "activewindowv2" => Ok(Self::ActiveWindowV2 {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "fullscreen" => {
                let enter_fullscreen = match data {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                Ok(Self::Fullscreen { enter_fullscreen })
            }
            "monitorremoved" => Ok(Self::MonitorRemoved {
                monitor: crate::types::MonitorName(data.to_string()),
            }),
            "monitorremovedv2" => {
                // The description is last and may contain commas.
                let mut split = data.splitn(3, ',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;
                let description = split.next().ok_or_else(malformed_data)?;

                Ok(Self::MonitorRemovedV2 {
                    id: crate::types::MonitorId(id),
                    name: crate::types::MonitorName(name.to_string()),
                    description: description.to_string(),
                })
            }
            "monitoradded" => Ok(Self::MonitorAdded {
                name: crate::types::MonitorName(data.to_string()),
            }),
            "monitoraddedv2" => {
                // The description is last and may contain commas.
                let mut split = data.splitn(3, ',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;
                let description = split.next().ok_or_else(malformed_data)?;

                Ok(Self::MonitorAddedV2 {
                    id: crate::types::MonitorId(id),
                    name: crate::types::MonitorName(name.to_string()),
                    description: description.to_string(),
                })
            }
            "createworkspace" => Ok(Self::CreateWorkspace {
                name: crate::types::WorkspaceName(data.to_string()),
            }),
            "createworkspacev2" => {
                let mut split = data.split(',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::CreateWorkspaceV2 {
                    id: crate::types::WorkspaceId(id),
                    name: crate::types::WorkspaceName(name.to_string()),
                })
            }
            "destroyworkspace" => Ok(Self::DestroyWorkspace {
                name: crate::types::WorkspaceName(data.to_string()),
            }),
            "destroyworkspacev2" => {
                let mut split = data.split(',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::DestroyWorkspaceV2 {
                    id: crate::types::WorkspaceId(id),
                    name: crate::types::WorkspaceName(name.to_string()),
                })
            }
            "moveworkspace" => {
                let mut split = data.split(',');

                let workspace = split.next().ok_or_else(malformed_data)?;
                let monitor = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::MoveWorkspace {
                    workspace: crate::types::WorkspaceName(workspace.to_string()),
                    monitor: crate::types::MonitorName(monitor.to_string()),
                })
            }
            "moveworkspacev2" => {
                let mut split = data.split(',');

                let workspace_id = split.next().ok_or_else(malformed_data)?;
                let workspace_id = workspace_id.parse().map_err(|_| malformed_data())?;
                let workspace_name = split.next().ok_or_else(malformed_data)?;
                let monitor = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::MoveWorkspaceV2 {
                    workspace_id: crate::types::WorkspaceId(workspace_id),
                    workspace_name: crate::types::WorkspaceName(workspace_name.to_string()),
                    monitor: crate::types::MonitorName(monitor.to_string()),
                })
            }
            "renameworkspace" => {
                let mut split = data.split(',');

                let id = split.next().ok_or_else(malformed_data)?;
                let id = id.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::RenameWorkspace {
                    id: crate::types::WorkspaceId(id),
                    name: crate::types::WorkspaceName(name.to_string()),
                })
            }
            "activespecial" => {
                let mut split = data.split(',');

                let workspace = split.next().ok_or_else(malformed_data)?;
                let monitor = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::ActiveSpecial {
                    workspace: crate::types::WorkspaceName(workspace.to_string()),
                    monitor: crate::types::MonitorName(monitor.to_string()),
                })
            }
            "activespecialv2" => {
                let mut split = data.split(',');

                let workspace_id = split.next().ok_or_else(malformed_data)?;
                let workspace_id = workspace_id.parse().map_err(|_| malformed_data())?;
                let workspace_name = split.next().ok_or_else(malformed_data)?;
                let monitor = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::ActiveSpecialV2 {
                    workspace_id: crate::types::WorkspaceId(workspace_id),
                    workspace_name: crate::types::WorkspaceName(workspace_name.to_string()),
                    monitor: crate::types::MonitorName(monitor.to_string()),
                })
            }
            "activelayout" => {
                // The layout name is last and may contain commas.
                let mut split = data.splitn(2, ',');

                let keyboard = split.next().ok_or_else(malformed_data)?;
                let layout = split.next().ok_or_else(malformed_data)?;

                Ok(Self::ActiveLayout {
                    keyboard: crate::types::KeyboardName(keyboard.to_string()),
                    layout: crate::types::LayoutName(layout.to_string()),
                })
            }
            "openwindow" => {
                // The title is last and may contain commas.
                let mut split = data.splitn(4, ',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let workspace = split.next().ok_or_else(malformed_data)?;
                let class = split.next().ok_or_else(malformed_data)?;
                let title = split.next().ok_or_else(malformed_data)?;

                Ok(Self::OpenWindow {
                    address,
                    workspace: crate::types::WorkspaceName(workspace.to_string()),
                    class: crate::types::WindowClass(class.to_string()),
                    title: crate::types::WindowTitle(title.to_string()),
                })
            }
            "closewindow" => Ok(Self::CloseWindow {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "kill" => Ok(Self::Kill {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "movewindow" => {
                let mut split = data.split(',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let workspace = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::MoveWindow {
                    address,
                    workspace: crate::types::WorkspaceName(workspace.to_string()),
                })
            }
            "movewindowv2" => {
                let mut split = data.split(',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let workspace_id = split.next().ok_or_else(malformed_data)?;
                let workspace_id = workspace_id.parse().map_err(|_| malformed_data())?;
                let workspace_name = split.next().ok_or_else(malformed_data)?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::MoveWindowV2 {
                    address,
                    workspace_id: crate::types::WorkspaceId(workspace_id),
                    workspace_name: crate::types::WorkspaceName(workspace_name.to_string()),
                })
            }
            "openlayer" => Ok(Self::OpenLayer {
                namespace: crate::types::Namespace(data.to_string()),
            }),
            "closelayer" => Ok(Self::CloseLayer {
                namespace: crate::types::Namespace(data.to_string()),
            }),
            "submap" => Ok(Self::Submap {
                submap: crate::types::SubmapName(data.to_string()),
            }),
            "changefloatingmode" => {
                let mut split = data.split(',');

                let window = split.next().ok_or_else(malformed_data)?;
                let window = window.parse().map_err(|_| malformed_data())?;
                let floating = split.next().ok_or_else(malformed_data)?;
                let floating = match floating {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::ChangeFloatingMode { window, floating })
            }
            "urgent" => Ok(Self::Urgent {
                window: data.parse().map_err(|_| malformed_data())?,
            }),
            "screencast" => {
                let mut split = data.split(',');

                let state = split.next().ok_or_else(malformed_data)?;
                let state = match state {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };
                let owner = split.next().ok_or_else(malformed_data)?;
                let owner = owner.parse().map_err(|_| malformed_data())?;

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::ScreenCast { state, owner })
            }
            "screencastv2" => {
                // The name can be a window title, so it is last and may contain commas.
                let mut split = data.splitn(3, ',');

                let state = split.next().ok_or_else(malformed_data)?;
                let state = match state {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };
                let owner = split.next().ok_or_else(malformed_data)?;
                let owner = owner.parse().map_err(|_| malformed_data())?;
                let name = split.next().ok_or_else(malformed_data)?;

                Ok(Self::ScreenCastV2 {
                    state,
                    owner,
                    name: name.to_string(),
                })
            }
            "windowtitle" => Ok(Self::WindowTitle {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "windowtitlev2" => {
                // The title is last and may contain commas.
                let mut split = data.splitn(2, ',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let title = split.next().ok_or_else(malformed_data)?;

                Ok(Self::WindowTitleV2 {
                    address,
                    title: crate::types::WindowTitle(title.to_string()),
                })
            }
            "togglegroup" => {
                let mut split = data.split(',');

                let state = split.next().ok_or_else(malformed_data)?;
                let state = match state {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };
                let addresses = split
                    .map(|address| address.parse().map_err(|_| malformed_data()))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self::ToggleGroup { state, addresses })
            }
            "moveintogroup" => Ok(Self::MoveIntoGroup {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "moveoutofgroup" => Ok(Self::MoveOutOfGroup {
                address: data.parse().map_err(|_| malformed_data())?,
            }),
            "ignoregrouplock" => {
                let enabled = match data {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                Ok(Self::IgnoreGroupLock { enabled })
            }
            "lockgroups" => {
                let locked = match data {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                Ok(Self::LockGroups { locked })
            }
            "configreloaded" => {
                if !data.is_empty() {
                    return Err(malformed_data());
                }

                Ok(Self::ConfigReloaded {})
            }
            "pin" => {
                let mut split = data.split(',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let pinned = split.next().ok_or_else(malformed_data)?;
                let pinned = match pinned {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::Pin { address, pinned })
            }
            "minimized" => {
                let mut split = data.split(',');

                let address = split.next().ok_or_else(malformed_data)?;
                let address = address.parse().map_err(|_| malformed_data())?;
                let minimized = split.next().ok_or_else(malformed_data)?;
                let minimized = match minimized {
                    "0" => false,
                    "1" => true,
                    _ => return Err(malformed_data()),
                };

                if split.next().is_some() {
                    return Err(InvalidEvent::MalformedData {
                        for_event: event_kind.to_string(),
                        data: data.to_string(),
                    });
                }

                Ok(Self::Minimized { address, minimized })
            }
            "bell" => {
                let address = data.parse().map_err(|_| malformed_data())?;

                Ok(Self::Bell { address })
            }
            other => Err(InvalidEvent::UnknownKind { kind: other.to_string() }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InvalidEvent {
    MalformedData { for_event: String, data: String },
    MalformedEvent { event: String },
    UnknownKind { kind: String },
}

impl std::fmt::Display for InvalidEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedData { for_event, data } => write!(f, "Malformed event data for event {for_event}: {data}"),
            Self::MalformedEvent { event } => write!(f, "Malformed event: {event}"),
            Self::UnknownKind { kind } => write!(f, "Unknown event kind: {kind}"),
        }
    }
}

impl std::error::Error for InvalidEvent {}
