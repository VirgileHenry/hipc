pub mod monitors;

/// Hyprland socket to send commands.
///
/// This allows to keep the socket path in memory when we want to ask for multiple commands.
pub struct HyprlandSocket(String);

impl HyprlandSocket {
    /// Creates a new Hyprland event socket.
    ///
    /// This connects to `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock`
    pub fn connect() -> std::io::Result<Self> {
        let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map_err(|e| std::io::Error::other(format!("Failed to get the XDG_RUNTIME_DIR env var: {e}")))?;
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|e| std::io::Error::other(format!("Failed to get the XDG_RUNTIME_DIR env var: {e}")))?;

        let socket_path = format!("{xdg_runtime_dir}/hypr/{his}/.socket.sock");

        Ok(Self(socket_path))
    }

    fn socket(&self) -> std::io::Result<std::os::unix::net::UnixStream> {
        std::os::unix::net::UnixStream::connect(&self.0)
    }

    /// Request all monitors with the `monitors` command.
    pub fn monitors(&self) -> std::io::Result<Vec<crate::commands::monitors::Monitor>> {
        use std::io::Read;
        use std::io::Write;

        let mut socket = self.socket()?;

        socket.write_all(b"j/monitors")?;
        let mut out = String::new();
        socket.read_to_string(&mut out)?;

        serde_json::from_str(&out).map_err(std::io::Error::other)
    }
}

/// The `hyprctl monitors` command.
pub fn monitors() -> std::io::Result<Vec<crate::commands::monitors::Monitor>> {
    HyprlandSocket::connect()?.monitors()
}
