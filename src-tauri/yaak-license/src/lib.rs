use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;
pub mod error;
mod license;

use crate::commands::{activate, check, deactivate};
pub use license::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("yaak-license")
        .invoke_handler(generate_handler![check, activate, deactivate])
        .build()
}

pub(crate) fn get_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}
