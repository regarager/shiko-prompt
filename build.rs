use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let config_file =
        PathBuf::from(env::var("SHIKO_THEME").unwrap_or("./themes/default.json".to_string()));

    let canonical_path = config_file.canonicalize().expect("config file not found");

    // hotloading
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("config.rs");

    let mut content = String::new();

    content.push_str("use crate::config_loader::ConfigSource;\n");

    if env::var("SHIKO_HOTLOAD").is_ok() {
        content.push_str(&format!(
            "pub static CONFIG_SOURCE: ConfigSource = ConfigSource::Hotload(\"{}\");",
            canonical_path.display()
        ))
    } else {
        let config_content =
            fs::read_to_string(&canonical_path).expect("failed to read config file");

        content.push_str(&format!(
                "pub static CONFIG_SOURCE: ConfigSource = ConfigSource::Hardcoded(r##\"{config_content}\"##);"
            ))
    }

    let _ = fs::write(&dest_path, content);

    println!("cargo:rerun-if-env-changed=SHIKO_THEME");
    println!("cargo:rerun-if-changed={}", canonical_path.display());
}
