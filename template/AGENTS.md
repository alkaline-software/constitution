# AGENTS.md

## What this is

**Starter** is the template every Alkaline Software app begins from: a Rust and GPUI desktop app for macOS, Windows and Linux with the studio's window, settings, motion, languages, icons and release already in place. Its page (`Shell::render_page`) is a sample of every pattern; a product replaces it and keeps the rest.

## Rules that hold everywhere

- **Nothing jumps.** What appears, goes, opens, closes, grows or moves does so over a moment, with the pattern its neighbors use (`widgets.rs`, `menu.rs`, `shell.rs`). No pause at the end of a motion.
- **A motion that repeats runs on the window's clock** (`ambient::phase`), never `Animation::repeat`.
- **Every scroller that goes up and down names itself** to the router as it is drawn (`Scrollers::name`), or a flick in it scrolls whatever the pointer reaches next.
- **A button is `choice_pill`**, a pill with an icon before its word. The toolkit's `Button` is only for an icon alone.
- **Every word the window shows goes through `i18n::t`** and is in `locales/words.json` in every language; `cargo test -p starter-app` fails otherwise.
- **A key is drawn on caps** through `key_text`, `kbd_hint` or `tip`, named as the Mac has it and bound as `secondary-<key>`.
- **Every child process starts through `starter_core::process::command`**, never `Command::new`.
- **Per-system calls live in `sys.rs`**, each with a body for every system.
- **One copy of gpui.** The pin is in `Cargo.lock`. Changes to the toolkit go in `vendor/gpui-component/`, marked `(Studio addition.)` and listed in its `UPSTREAM.md`. Build from the root, never inside `vendor/`.

## Layout

```
Cargo.toml                 the workspace and the one version
crates/starter-core/       everything without a window: paths, config, process, update, the CLI
crates/starter-app/        the window
  src/main.rs                menus, key bindings, the main window
  src/shell.rs               the main view: frame, sidebar, card, the sample page
  src/settings.rs            the settings window: rail, group, line
  src/widgets.rs             the shared drawing: choice_pill, key_text, tip, page_in, Motion
  src/menu.rs                the app's own menu, in and out in motion
  src/scroll.rs              a gesture stays in the scroller it began in
  src/ambient.rs             the window's clock
  src/look.rs                appearance, accents, code's inks
  src/fonts.rs               the faces carried
  src/assets.rs              the icons and pictures built in
  src/file_icons.rs          Catppuccin's file icons
  src/i18n.rs                the window's words in eight languages
  src/sys.rs                 per-system calls
  src/csd.rs                 the window's own buttons where the system draws none
  src/ui_state.rs            what the window remembers
  assets/ themes/ locales/   icons, fonts, the app icon; the palette; the words
  macos/ windows/ linux/     what each system's package needs
vendor/                    the toolkit and the blocks crate, each with an UPSTREAM.md
scripts/                   the dev build, the icon, the release
docs/                      release.md, probing.md
```

## Running it

```
cargo test -p starter-core -p starter-app
cargo build -p starter-app && scripts/dev-app.sh
```

One dev build at a time. After a change to the app, build it and run `scripts/relaunch.sh` as the last command.
