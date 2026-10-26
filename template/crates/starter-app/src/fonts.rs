//! The faces the window carries, and which face each place uses.
//!
//! The chrome is the system's face and is never set here. The app's name
//! (the sidebar's wordmark, About's title card) is Optima where the
//! system has it, else Belleza, a free face with the same flared strokes,
//! carried for Windows and Linux. A product whose content is words adds
//! its content faces and its monospace faces here, each a static file a
//! weight under a license that lets an app carry it, with that license
//! beside the files in `assets/fonts/`: handed a variable font, gpui holds
//! one face of it and draws every bold as regular.

use std::borrow::Cow;

use gpui::{App, SharedString};

const FACES: &[&[u8]] = &[include_bytes!("../assets/fonts/Belleza-Regular.ttf")];

/// Load the carried faces. Called once at start.
pub fn install(cx: &mut App) {
    let faces = FACES.iter().map(|b| Cow::Borrowed(*b)).collect();
    if let Err(e) = cx.text_system().add_fonts(faces) {
        eprintln!("starter: the carried faces did not load: {e}");
    }
}

/// The app's name's face. Optima has only regular and bold, so the
/// wordmark draws its regular twice half a pixel apart for a weight
/// between (`widgets::wordmark`).
pub fn wordmark_family(cx: &App) -> SharedString {
    let names = cx.text_system().all_font_names();
    if names.iter().any(|n| n == "Optima") { "Optima".into() } else { "Belleza".into() }
}
