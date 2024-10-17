//! Saving/loading preferences (small amounts of user data)
//!
//! Web implementation uses [local storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
#![warn(missing_docs)]

use serde::{de::DeserializeOwned, Serialize};

fn serialize<T: Serialize>(value: &T) -> String {
    ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::new())
        .expect("Failed to serialize save")
}

fn deserialize<T: DeserializeOwned>(s: &str) -> Result<T, ron::de::SpannedError> {
    ron::from_str(s)
}

/// Base path where preferences are going to be saved/loaded from
pub fn base_path() -> std::path::PathBuf {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            ".".into() // TODO: detect app name by url?
        } else if #[cfg(target_os = "android")] {
             batbox_android::app().external_data_path().unwrap().join(".preferences")
        } else {
            let exe = std::env::current_exe().expect("Failed to find current exe");
            let app_name = exe.file_stem().unwrap();
            if let Some(dirs) =
                directories::ProjectDirs::from("", "", app_name.to_str().expect("Exe name is invalid"))
            {
                return dirs.preference_dir().to_path_buf();
            }
            if let Some(dir) = exe.parent() {
                return dir.to_path_buf();
            }
            std::env::current_dir().unwrap()
        }
    }
}

/// Save given value for given key
pub fn save<T: Serialize>(key: &str, value: &T) {
    let base_path = base_path();
    let path = base_path.join(key);
    #[cfg(target_arch = "wasm32")]
    {
        let path = path.to_str().unwrap();
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            if let Err(e) = storage.set_item(path, &serialize(value)) {
                let _ = e; // TODO: error?
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::Write;
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            log::error!("Failed to create preferences base path: {}", e);
            return;
        }
        let path = &path;
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(e) => {
                log::error!("Failed to create {:?}: {}", path, e);
                return;
            }
        };
        if let Err(e) = file.write_all(serialize(value).as_bytes()) {
            log::error!("Failed to save {:?}: {}", path, e);
        }
    }
}

/// Load value for given key
pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let base_path = base_path();
    let path = base_path.join(key);
    #[cfg(target_arch = "wasm32")]
    {
        let path = path.to_str().unwrap();
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            match storage
                .get_item(path)
                .ok()
                .flatten()
                .map(|s| deserialize(&s))
            {
                Some(Ok(value)) => Some(value),
                Some(Err(e)) => {
                    log::error!("Failed to deserialize {:?}: {}", path, e);
                    None
                }
                None => None,
            }
        } else {
            None
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = &path;
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                log::warn!("Failed to open {:?}: {}", path, e);
                return None;
            }
        };
        let mut contents = String::new();
        use std::io::Read;
        file.read_to_string(&mut contents)
            .expect("Failed to read save file");
        match deserialize(&contents) {
            Ok(value) => {
                log::debug!("Successfully loaded {:?}", path);
                Some(value)
            }
            Err(e) => {
                log::error!("Failed to deserialize {:?}: {}", path, e);
                None
            }
        }
    }
}
