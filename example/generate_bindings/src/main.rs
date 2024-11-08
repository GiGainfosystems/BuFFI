use std::fs;
use std::path::PathBuf;

fn main() {
    let mut config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_dir.push("api_config.toml");

    let toml_string = fs::read_to_string(&config_dir).expect("Config path does not exist");
    let config: buffi::Config = toml::from_str(&toml_string).expect("Could not read config toml");

    let mut include_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    include_dir.push("..");
    include_dir.push("buffi_example");
    include_dir.push("src");
    include_dir.push("include");

    if !include_dir.exists() {
        fs::create_dir_all(&include_dir).unwrap();
    }
    let include_dir = include_dir.canonicalize().unwrap();

    buffi::generate_bindings(include_dir.as_path(), config);
}
