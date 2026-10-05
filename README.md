# HIPC

Dead simple hyprland ipc protocol. It works, it's mine, it's so simple it's dumb

### Example

Quick example because "on est pas des bêtes":

```rust
/* Create a new hyprland event socket */
let socket = hipc::HyprlandEventSocket::connect();

/* Read will wait for incoming events and stop the current thread */
for event in socket.read() {
    match event {
        hipc::HyprlandEvent::FocusedMonitorV2 { monitor, workspace } => println!("Switched to monitor {monitor} (holding worskpace {workspace})"),
        other => {/* Whataver you want */}
    }
}
```
