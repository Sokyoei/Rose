use std::{path::PathBuf, sync::LazyLock};

pub fn get_project_dir() -> PathBuf {
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"));
}

pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| get_project_dir());

pub fn get_sokyoei_data_dir() -> PathBuf {
    return PathBuf::from(env!("SOKYOEI_DATA_DIR"));
}

pub static SOKYOEI_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| get_project_dir());
