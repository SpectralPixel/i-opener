fn main() {
    let mut connection = i3ipc::I3Connection::connect().unwrap();
    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let focused_ws = workspaces.into_iter().find(|ws| ws.focused).unwrap();

    match focused_ws.name.as_ref() {
        "1" => println!("foo"),
        "2" => println!("bar"),
        _ => println!("baz"),
    }
}
