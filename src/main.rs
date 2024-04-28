use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Default)]
struct Config {
    bindings: ron::Map,
    fallback: String,
}

fn main() {
    let config: Config = confy::load("i-opener", None).unwrap();

    let mut connection = i3ipc::I3Connection::connect().unwrap();
    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let focused_ws = workspaces.into_iter().find(|ws| ws.focused).unwrap();

    match focused_ws.name.as_ref() {
        "1" => println!("foo"),
        "2" => println!("bar"),
        _ => println!("baz"),
    }
}
