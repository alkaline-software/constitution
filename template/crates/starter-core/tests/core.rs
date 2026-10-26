//! The core's tests. Each points `STARTER_HOME` at a temp folder of its
//! own (`isolated`), so none reads or writes the person's files.

use std::sync::Mutex;

use starter_core::{config::Config, paths, update};

/// Tests run on threads of one process and share its environment.
static ENV: Mutex<()> = Mutex::new(());

fn isolated() -> (tempfile::TempDir, std::sync::MutexGuard<'static, ()>) {
    let guard = ENV.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir().expect("a temp folder");
    std::env::set_var("STARTER_HOME", dir.path());
    std::env::remove_var("STARTER_APPEARANCE");
    (dir, guard)
}

#[test]
fn a_missing_or_broken_file_is_the_defaults() {
    let (_dir, _g) = isolated();
    assert_eq!(Config::load(), Config::default());
    std::fs::write(paths::config_file(), "{ not json").unwrap();
    assert_eq!(Config::load(), Config::default());
}

#[test]
fn a_file_with_a_field_missing_still_loads() {
    let (_dir, _g) = isolated();
    std::fs::write(paths::config_file(), r#"{"app": {"accent": "jade"}}"#).unwrap();
    let cfg = Config::load();
    assert_eq!(cfg.app.accent, "jade");
    assert_eq!(cfg.app.appearance, "system");
}

#[test]
fn an_override_is_never_written_back() {
    let (_dir, _g) = isolated();
    std::env::set_var("STARTER_APPEARANCE", "dark");
    assert_eq!(Config::load().app.appearance, "dark");
    Config::edit(|c| c.app.accent = "rose".into()).unwrap();
    std::env::remove_var("STARTER_APPEARANCE");
    let cfg = Config::load();
    assert_eq!(cfg.app.appearance, "system");
    assert_eq!(cfg.app.accent, "rose");
}

#[test]
fn a_write_leaves_no_temp_file() {
    let (dir, _g) = isolated();
    paths::write_json(&paths::state_dir().join("x.json"), &serde_json::json!({"a": 1})).unwrap();
    let names: Vec<String> = std::fs::read_dir(dir.path().join("state")).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    assert_eq!(names, ["x.json"]);
}

#[test]
fn the_version_is_read_off_the_redirect() {
    assert_eq!(update::version_from_location("https://github.com/o/r/releases/tag/v1.2.3").as_deref(), Some("1.2.3"));
    assert_eq!(update::version_from_location("https://github.com/o/r/releases/tag/1.2.3/").as_deref(), Some("1.2.3"));
    assert_eq!(update::version_from_location("https://github.com/o/r/releases"), None);
    assert!(update::is_newer("0.10.0", "0.9.9"));
    assert!(!update::is_newer("1.0", "1.0.0"));
}

#[test]
fn every_system_has_an_installer_name() {
    let name = update::asset_name().expect("this system has an installer");
    assert!(name.starts_with(update::APP), "{name}");
}
