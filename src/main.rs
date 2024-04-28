fn main() {
    let mut connection = i3ipc::I3Connection::connect().unwrap();
    let workspaces = connection.get_workspaces().unwrap();

    // debug, see if workspaces are gotten correctly
    for w in workspaces.workspaces {
        println!("workspace: {} - {}", w.num, w.name);
    }
}
