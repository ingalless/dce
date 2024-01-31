use std::{self, env, fs, process::Command};

use yaml_rust::YamlLoader;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();

    let cwd = env::current_dir().expect("Could not read cwd");
    let file: String =
        match fs::read_to_string(format!("{}/docker-compose.yml", cwd.to_string_lossy())) {
            Ok(v) => v,
            Err(_) => fs::read_to_string(format!("{}/compose.yml", cwd.to_string_lossy()))
                .unwrap_or("".to_string()),
        };

    let yamls = &YamlLoader::load_from_str(&file).unwrap();
    
    let container_name = match yamls.get(0) {
        Some(yaml) => yaml["services"]["web"]["container_name"].as_str().unwrap_or(""),
        None => ""
    };

    let mut args: Vec<_> = env::args().collect();
    args.remove(0);

    let mut command = Command::new("docker");

    command.arg("exec")
        .arg("-it");

    if container_name != "" {
        command.arg(container_name);
    }
    command.args(args);

    let mut child = command.spawn()
        .expect("Failed to execute");

    child.wait().unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();
}
