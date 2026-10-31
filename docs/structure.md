# Structure

How an Alkaline Software product is put together: the stack, the workspace, the folders, what goes where in the code, and the rules that keep it building on three systems. `template/` is this file as code: a product that builds, tests and runs, which `scripts/new-product.sh` copies and renames into a new product.

## Rules

- **Rust and GPUI.** The window is drawn by gpui (Zed's toolkit) with the vendored `gpui-component`, on the GPU on all three systems (Metal, DirectX, wgpu). Speed on long content is why: a list or a document of tens of megabytes opens and scrolls without a stutter. gpui is pre-1.0, and its breaking changes are taken as they come.
- **Two crates.** `crates/<app>-core` is everything without a window: reading, storing, searching, the child processes, the update, a CLI. `crates/<app>-app` is the window. The core has no UI dependency, so it is tested on every system without a display and a script can drive it from a terminal.
- **One copy of gpui.** The pin is in `Cargo.lock`, never a `rev` of the product's own on a gpui dependency: gpui-component depends on the unpinned zed git source, and a `rev` makes a second copy it does not build against. `cargo update -p gpui --precise <rev>` moves it. The template's lock holds the pin the products build against.
- **The toolkit is vendored, and every change to it is listed.** `vendor/gpui-component/` holds the crates the product uses, `[patch]`ed over the git source so the manifests still say where the code came from. Every change in it is marked `(Studio addition.)` in the source and numbered in `vendor/gpui-component/UPSTREAM.md` with what upstream did, or it is lost at the next move to a newer upstream. A fork on GitHub was the alternative; one repository was preferred.
- **Build from the root.** Never inside `vendor/`: a vendored crate is a workspace of its own, and cargo there resolves its own lock against another gpui and writes a second `target/` of gigabytes. The root `Cargo.toml` excludes `vendor` from the workspace.
- **One version**, `[workspace.package] version` in the root `Cargo.toml`; both crates inherit it (`docs/workflow.md`).
- **The dev build is usable.** `[profile.dev]` at `opt-level = 1` and every dependency at 2: gpui's tree is large, and at 0 the window stutters. A dependency that checks its own work at every step in debug (a formatter, a parser) is built as a release has it.
- **Every child process starts through the core's `process::command`,** never `Command::new`. A release build on Windows has no console, and a console program started from it opens a console window of its own unless it is started with `CREATE_NO_WINDOW`, which `command` sets.
- **Per-system work goes in one file,** `sys.rs`, each function with a body for every system. No macOS command is shelled out from anywhere else (`docs/platforms.md`).
- **Every word the window shows goes through `t`** and is in the language table in every language; a test fails otherwise (`docs/design.md`, Languages).
- **Every icon the code names is carried;** a test fails otherwise.
- **Nothing personal is in the repository**: no signing identity, no credential, no path under one person's home. Scripts find them in the keychain or the environment.
- **Tests never touch the person's data.** The core's paths come from an environment variable (`<APP>_HOME`, and the same for any other tool's folder it reads), and the tests point each at a temp folder. CI sets them too.

## The repository

| Path | What it is |
|---|---|
| `AGENTS.md` | what the product is, the rules every session follows, where to read next (`docs/docs.md`) |
| `CLAUDE.md` | only for an agent that reads no `AGENTS.md`: a symlink to it, or one line importing it |
| `README.md` | a short introduction for GitHub that sends the reader to the product's site |
| `CHANGELOG.md` | one section per release, for the person installing it (`docs/docs.md`) |
| `LICENSE` | MIT |
| `Cargo.toml`, `Cargo.lock` | the workspace, the one version, the gpui pin |
| `.cargo/config.toml` | Windows links the C runtime in (`+crt-static`, `docs/platforms.md`) |
| `crates/<app>-core/` | everything without a window, its CLI in `src/bin/<app>-core.rs`, its tests in `tests/` |
| `crates/<app>-app/` | the window |
| `crates/<app>-app/assets/` | `icons/` (Tabler's under the toolkit's names, and the ones drawn by hand), `fonts/` (each face with its license), `icon/` (the app icon in every size), `catppuccin/` for a file tree |
| `crates/<app>-app/themes/<app>.json` | the palette, one config per appearance |
| `crates/<app>-app/locales/words.json` | every phrase in every language |
| `crates/<app>-app/macos/`, `windows/`, `linux/` | what each system's package needs: the `Info.plist` additions, the Store's manifest and tiles, the snap's metadata and desktop file |
| `vendor/` | the toolkit and any crate that had to be changed, each with an `UPSTREAM.md` |
| `docs/` | how each part works and its traps, one file a part (`docs/docs.md`); `release.md`, how to cut a release; `probing.md`, how to look at the window from a script |
| `scripts/` | the dev build, the icon, the release (`docs/workflow.md`) |
| `.github/workflows/` | `rust.yml` on every push, `release.yml` on a tag |

Nothing else at the root. A plan or a roadmap kept as a file goes stale the week it is written; what is so now is in `docs/`, and what each release changed is in `CHANGELOG.md`.

## The app crate

The template's files, each with one job; a product keeps the names, so a reader of one product finds their way in another:

| File | Job |
|---|---|
| `main.rs` | the menus, the key bindings, the main window opened at its remembered place, the probe's variables |
| `shell.rs` | the main view: the frame, the sliding sidebar, the card, the strip's buttons; the page, which a product replaces |
| `settings.rs` | the settings window: the rail, `group`, `line`, the sections, About with the updates |
| `widgets.rs` | the shared drawing: `choice_pill`, `small_pill`, `key_text`, `kbd_hint`, `tip`, `page_in`, `fade_in`, `float_shadow`, `wordmark`, and `Motion` (`presence`, `switch`, `segmented`) |
| `menu.rs` | the app's own menu, coming in and going out in motion |
| `scroll.rs` | the router that keeps a gesture in the scroller it began in |
| `ambient.rs` | the window's clock for whatever moves while something works |
| `look.rs` | the appearance, the accents (`ACCENTS`), code's inks |
| `fonts.rs` | the faces carried, and which face each place uses |
| `assets.rs` | the icons and pictures built in, before the toolkit's own |
| `file_icons.rs` | Catppuccin's icon for a file or folder by its name |
| `i18n.rs` | `t`, `tf`, `count`, `phrase`, `weave`, and the language chosen |
| `sys.rs` | the per-system calls, and the window's picture for a probe |
| `csd.rs` | the window's own buttons and edges where the system draws none |
| `ui_state.rs` | what the window remembers |

- **Work leaves the main thread.** Reading files, running git, a network call, a model's answer: each runs on gpui's background executor or a thread and comes back to the view. The window never waits on the disk.
- **Settings are written through one call** (`Config::edit`), which starts from the file and never from what `Config::load` returned, so an override from the environment is never saved.
- **The drawing helpers are shared, never copied.** A second button drawn by hand is a second pattern (`docs/design.md`, One way to draw each thing).

## The core crate

- **A CLI over every part** (`<app>-core paths | config | update | …`), so a part can be run and timed without the window, by a person or an agent. A timing subcommand over the largest inputs is how a product learns whether a speed-up is needed before building one.
- **State in one folder**, `~/.<app>`, from `paths.rs`, overridden by `<APP>_HOME`.
- **Writes are a temp file and a rename** (`paths::write_atomic`). A file the window reads is never seen half written.
- **Update** in `update.rs` (`docs/workflow.md`, The update inside the app).
- **A local model is run off the main thread and asked no more than it must.** Its lock is held for all of a load or a run, so nothing on the main thread takes it, even to look (`docs/design.md`, Frames). What is the same in every prompt is read once: the instructions before the last turn took three quarters of a short request, 685 ms of 900. A copy of the model's memory is kept once that part is read, and each request goes on from it while that part is the same. A rollback is no way to it: a model with a recurrent state refuses one. Such a change must leave what the model writes as it was, word for word.

## Probing

Every product can be driven from a script, so a change is looked at by an agent without the person's hands. The template's `docs/probing.md` lists its variables:

- A second copy runs with its own `<APP>_HOME`, never the copy the person is using.
- Variables pick what the copy shows: an appearance, a language, a section of the settings (`<APP>_APPEARANCE`, `<APP>_LANG`, `<APP>_SETTINGS`), and a list of steps (`<APP>_GO`).
- `<APP>_SHOT=<file>` writes a picture of the window once it is drawn, then quits. A process may take a picture of its own windows without Screen Recording access.
- A synthetic click does not reach a background window, so a picture of the state a click leads to does not show the click works. Say what was checked and what was not.
- Speed is measured before it is worked on, and again after, with a stopwatch in a copy that is taken out when it is done (`docs/probing.md`, Measuring). The first guess is often wrong: a change that looked right saved nothing until it was timed, and a cost that looked like a slow frame was a wait outside any frame.
