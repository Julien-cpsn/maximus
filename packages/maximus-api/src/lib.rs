use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub mod user;
pub mod models;
pub mod rooms;

pub static PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(|| ProjectDirs::from("com", "julien-cpsn", "maximus").expect("No valid home directory path could be retrieved from the operating system"));

pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let data_dir = PROJECT_DIRS.data_dir().to_path_buf();

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).expect(&format!("Could not create data directory: {}", data_dir.display()));
    }

    data_dir
});

pub static SESSION_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| DATA_DIR.join("session.json"));

pub static DATABASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let database_dir = DATA_DIR.join("db");

    if !database_dir.exists() {
        fs::create_dir(&database_dir).expect(&format!("Could not create db directory: {}", database_dir.display()));
    }

    database_dir
});

pub static AVATAR_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let avatars_dir = DATA_DIR.join("avatars");

    if !avatars_dir.exists() {
        fs::create_dir(&avatars_dir).expect(&format!("Could not create avatars directory: {}", avatars_dir.display()));
    }

    avatars_dir
});