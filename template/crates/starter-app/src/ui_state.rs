//! What the window remembers between launches: its bounds, whether the
//! sidebar was open, the page showing. `<home>/state/ui.json`, written
//! when any of it changes (the bounds at most every two seconds), at
//! quit, and when the window's close button closes it: on Windows and
//! Linux closing the last window ends the app without a quit's save. An
//! unreadable file means defaults, never a refusal to start.

use serde::{Deserialize, Serialize};
use starter_core::paths;

/// The window's bounds as `window.window_bounds()` gives them, never
/// `bounds()`: on Windows `bounds()` starts under the title bar while a
/// window is opened at a place with its frame above it, so a window
/// saved by `bounds()` opened half a title bar lower at each launch.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct UiState {
    pub window: Option<Rect>,
    pub sidebar_open: Option<bool>,
    /// The settings' section last shown, where they open again.
    pub settings_section: Option<String>,
}

fn file() -> std::path::PathBuf {
    paths::state_dir().join("ui.json")
}

impl UiState {
    pub fn load() -> Self {
        paths::read_json(&file()).and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(v) = serde_json::to_value(self) {
            let _ = paths::write_json(&file(), &v);
        }
    }
}
