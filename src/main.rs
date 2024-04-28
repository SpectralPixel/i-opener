use serde::{Deserialize, Serialize};

const APP_NAME: &str = "i-opener";

#[derive(Deserialize, Serialize, Default)]
struct Config {
    bindings: ron::Map,
    fallback: String,
}

fn main() {
    let config_file_path = confy::get_configuration_file_path(APP_NAME, None).unwrap();
    let config_file_path = config_file_path.to_string_lossy();
    println!("Configuration file can be found at: {}", config_file_path);

    let config: Config = confy::load(APP_NAME, None).unwrap();

    let mut connection = i3ipc::I3Connection::connect().unwrap();
    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let focused_ws = workspaces.into_iter().find(|ws| ws.focused).unwrap();

    let app_to_open = config.bindings.iter().find(|app| focused_ws.name == *app.0.clone().into_rust::<String>().unwrap());
    let app_to_open = match app_to_open {
        Some(app) => app.1.clone().into_rust::<String>().unwrap(),
        None => config.fallback,
    };

    let command = format!("exec {}", app_to_open);
    connection.run_command(&command).unwrap();
}
