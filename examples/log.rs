fn main() -> std::io::Result<()> {
    let mut socket = hipc::HyprlandEventSocket::connect()?;
    loop {
        let event = socket.read()?;
        println!("{event:?}")
    }
}
