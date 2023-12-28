use std::{self, env, fs, process::Command};

use yaml_rust::YamlLoader;

fn main() {
    let cwd = env::current_dir().expect("Could not read cwd");
    let file: String =
        match fs::read_to_string(format!("{}/docker-compose.yml", cwd.to_string_lossy())) {
            Ok(v) => v,
            Err(_) => fs::read_to_string(format!("{}/compose.yml", cwd.to_string_lossy()))
                .expect("Could not find a compose file"),
        };

    let yaml = &YamlLoader::load_from_str(&file).unwrap()[0];

    let container_name = yaml["services"]["web"]["container_name"]
        .as_str()
        .expect("The expected keys were not found in the compose file");

    let mut args: Vec<_> = env::args().collect();
    args.remove(0);

    crossterm::terminal::enable_raw_mode().unwrap();

    let mut child = Command::new("docker")
        .arg("exec")
        .arg("-it")
        .arg(container_name)
        .args(args)
        .spawn()
        .expect("Failed to execute");

    child.wait().unwrap();
}
