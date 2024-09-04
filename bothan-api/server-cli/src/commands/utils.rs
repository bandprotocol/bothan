use std::path::PathBuf;

pub fn bothan_home_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}
