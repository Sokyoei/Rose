use std::{path::PathBuf, sync::LazyLock};

pub fn get_project_dir() -> PathBuf {
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"));
}

pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| get_project_dir());
