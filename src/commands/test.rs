use super::*;

#[test]
fn test_all_commands() {
    match active_window() {
        Ok(_) => {}
        Err(e) => panic!("Expected active window, found error: {e}"),
    }
    match monitors() {
        Ok(_) => {}
        Err(e) => panic!("Expected monitors, found error: {e}"),
    }
    match workspace() {
        Ok(_) => {}
        Err(e) => panic!("Expected workspaces, found error: {e}"),
    }
}
