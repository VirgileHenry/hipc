use super::*;

#[test]
fn test_all_commands() {
    match active_window() {
        Ok(_) => {}
        Err(e) => panic!("Expected active window, found error: {e}"),
    }
    match clients() {
        Ok(_) => {}
        Err(e) => panic!("Expected clients, found error: {e}"),
    }
    match monitors() {
        Ok(_) => {}
        Err(e) => panic!("Expected monitors, found error: {e}"),
    }
    match workspaces() {
        Ok(_) => {}
        Err(e) => panic!("Expected workspaces, found error: {e}"),
    }
}
