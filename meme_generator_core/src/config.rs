use std::{env, fs, path::PathBuf, sync::LazyLock};

use directories::UserDirs;

pub fn meme_home() -> PathBuf {
    match env::var("MEME_HOME") {
        Ok(value) => PathBuf::from(value),
        Err(_) => {
            let user_dirs = UserDirs::new().unwrap();
            user_dirs.home_dir().join(".meme_generator")
        }
    }
}

pub static MEME_HOME: LazyLock<PathBuf> = LazyLock::new(meme_home);

pub fn read_config_file() -> String {
    let config_path = MEME_HOME.join("config.toml");
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|_| {
                eprintln!("Failed to create config directory");
            });
            fs::write(&config_path, "").unwrap_or_else(|_| {
                eprintln!("Failed to create config file");
            });
        }
    }
    if config_path.exists() {
        return fs::read_to_string(config_path).unwrap_or_else(|_| {
            eprintln!("Failed to read config file, using default config");
            String::new()
        });
    }
    String::new()
}
