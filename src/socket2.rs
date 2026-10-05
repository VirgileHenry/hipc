/// Hyprland second socket, where all events are sent.
pub struct HyprlandEventSocket(std::io::BufReader<std::os::unix::net::UnixStream>);

impl HyprlandEventSocket {
    /// Creates a new Hyprland event socket.
    ///
    /// This connects to `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock`
    pub fn connect() -> std::io::Result<Self> {
        let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map_err(|e| std::io::Error::other(format!("Failed to get the XDG_RUNTIME_DIR env var: {e}")))?;
        let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|e| std::io::Error::other(format!("Failed to get the XDG_RUNTIME_DIR env var: {e}")))?;

        let socket_path = format!("{xdg_runtime_dir}/hypr/{his}/.socket2.sock");
        let socket = std::os::unix::net::UnixStream::connect(socket_path)?;

        let reader = std::io::BufReader::new(socket);
        Ok(Self(reader))
    }

    /// Read the next event from the socket.
    ///
    /// This will call `[std::io::BufReader::read_line]` from the socket,
    /// which will cause to block the current thread.
    pub fn read(&mut self) -> std::io::Result<crate::HyprlandEvent> {
        use std::io::BufRead;
        use std::str::FromStr;

        let mut buffer = String::new();
        self.0.read_line(&mut buffer)?;

        match crate::HyprlandEvent::from_str(&buffer) {
            Ok(event) => Ok(event),
            Err(e) => Err(std::io::Error::other(format!("Failed to parse hyprland event: {e}"))),
        }
    }
}
