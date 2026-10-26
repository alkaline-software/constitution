# Platforms

How a product works on macOS, Windows and Linux, each on Intel-class and ARM machines: what each system needs, what is gated, and how each is checked before a release. The template builds for all six, and its CI tests on all six. A Mac-only product (a menu bar app) reads the Mac part and the rules, and keeps the code ready for the others: nothing in it assumes the Mac where a gate would do.

## Rules

- **Six builds.** macOS on Apple silicon and Intel; Windows on x64 and ARM; Linux on x64 and ARM. Every Windows or Linux VM on an Apple silicon Mac is ARM, and so are Snapdragon laptops: an x64-only build does not run at all on ARM Linux, and on ARM Windows only under emulation.
- **Everything Unix-only is gated with a stated fallback.** A `#[cfg(unix)]` has a `#[cfg(not(unix))]` beside it that says what happens instead (refuse, fall through to another way, do nothing), and the product's `docs/platform.md` lists each gate and its fallback.
- **Per-system calls live in `sys.rs`** (`docs/structure.md`): open a file, reveal it, the person's name, keep the display on (an IOKit assertion on a Mac, the thread's execution state on Windows, `systemd-inhibit` on Linux). Nothing else shells out to `open`, `osascript` or `id`.
- **No console window on Windows.** The release build sets `windows_subsystem = "windows"`, and every child process starts through `process::command`, which sets `CREATE_NO_WINDOW`.
- **Windows links the C runtime in** (`+crt-static` in `.cargo/config.toml`). Linked as a DLL it wants `VCRUNTIME140.dll`, which a fresh Windows does not have, and the app stopped at launch with a system error. A C dependency built another way than through the `cc` crate follows the same flag.
- **Paths are the system's.** `PATH` is split with `std::env::split_paths`; a tool is looked for under each system's names (`claude.exe` and an npm `.cmd` shim on Windows) and its installers' folders after `PATH`; a test's temp folder on Windows is `C:/Users/...`, and a parser that takes `x:y` for `host:path` reads it as a host "C". One letter before the colon is a drive.
- **A key is named as the Mac has it**, and bound as `secondary-<key>`, which gpui makes ⌘ on a Mac and Ctrl elsewhere. Where the window draws a key, ⌘ and ⌥ are the words Ctrl and Alt off a Mac (`docs/design.md`, Icons in the window).
- **The tests run on all six in CI** (`rust.yml`), since that is the only place most of them run off a Mac. A tag waits for that run to go green (`docs/workflow.md`).

## macOS

- **The bundle** is made by cargo-packager from `[package.metadata.packager]` in the app's manifest, with `macos/Info.plist` merged in (the languages, `LSUIElement` for a menu bar app).
- **The dev build is a bundle too** (`scripts/dev-app.sh`): `target/debug/dev.noindex/<App>.app`, opened through the system, signed with the developer's identity when the keychain has one. A bare executable opened from the Dock or Finder runs inside the default terminal app and every child it starts inherits that terminal's marks; a build signed by nobody is a new app to Privacy & Security after every build. `.noindex` keeps Spotlight from listing a second copy.
- **The window's own background** is set to the frame's color (`sys::set_window_ground`), or it is near white under a dark frame while the window is drawn or resized.
- **The installer** is a disk image, signed with the Developer ID, notarized and stapled (`docs/workflow.md`, The disk image).
- **The app's icon** is `.icns` at the full square, since macOS 26 masks it to its own rounded square (`docs/design.md`, The app icon).

## Windows

- **The installer is Velopack's** `Setup.exe` (`vpk pack`, its pack id `<Org>.<App>`): no wizard and no button, a splash while it puts the app in `%LOCALAPPDATA%\<pack id>`, then the app opens. `main` answers Velopack's hooks first (`VelopackApp`); the crate and `vpk` in `release.yml` are pinned to one version and move together. NSIS's installer was tried and dropped: a wizard of Windows 2000's look, and a repair question when an app was already in its folder.
- **The pack id is forever.** It names the install folder and the update packages (`<pack id>-<version>-win-<arch>-full.nupkg`); changing it leaves every installed copy without updates. Choose it once, before the first Velopack release.
- **Signing** is Azure Artifact Signing in CI, once its secrets are set (`docs/workflow.md`, Signing on Windows). Unsigned, SmartScreen stops the installer with "Windows protected your PC".
- **The Microsoft Store's copy** is an `.msix` built in the same release run, at full trust with write virtualization off (`unvirtualizedResources`), so a shell started in the app installs into the real AppData. The Store signs it and updates it, so it shows no Updates in the settings (`update::from_store`, by its `WindowsApps` folder).
- **The window's place** is saved from `window_bounds()`, never `bounds()`: on Windows `bounds()` starts under the title bar, and a window saved by it opened half a title bar lower at each launch. And closing the last window ends the app without a quit's save, so what the window remembers is written when its close button is pressed.
- **The scrollbar** is a thin line until the pointer is on it, as Windows 11's own is, and its thumb keeps its length at the ends, since there is no rubber band to show (the vendored `scrollbar.rs`).
- **A thread has one megabyte of stack** (eight on a Mac and Linux). A reader that recurses as deep as a document is nested runs on a thread of its own with a deep stack, and `release-check.sh` runs the core tests once more with `RUST_MIN_STACK=1048576`.
- **Git for Windows has `core.autocrlf` on.** A test's own repository sets it off with `git config`, not `-c`, since the code under test runs git itself.
- **A file name's trailing dot is ignored**: `main.rs.` answers as `main.rs`.

## Linux

- **Two packages**: a `.deb` for Debian and Ubuntu, which App Center installs at a double click, and an AppImage for every other distribution. A downloaded AppImage cannot be opened from Files until it is made executable, and an x64 one never runs on ARM; the site's download names both.
- **The Snap Store's copy** is a snap of the same binary with classic confinement (a product that runs the person's shell or reads their files in their home cannot be strict), packed by `snap pack` with no build container. App Center warns "Potentially unsafe" about every `.deb` opened from a file, whoever made it; the snap has no warning, and snapd keeps it current. Children started from it get none of snapd's `SNAP*` variables.
- **System libraries** for the build are the compilers, fontconfig and freetype, Wayland, X11 and xkbcommon, Vulkan and its software driver, and ALSA, as the template's `rust.yml` installs them on the Ubuntu runners, and its `.deb` names what the app needs at run time.
- **The window draws its own buttons** where the system draws none (`Decorations::Client`: GNOME on Wayland): minimize, maximize and close in the system's order and on its side (GNOME's `button-layout`), and edges to resize by (`csd.rs`). A window the system decorates (X11, KDE) gets nothing from it.
- **`app_id` is the desktop file's name** (`<App>`), or the shell shows a blank icon for the running app.
- **The update** replaces the AppImage in place (`APPIMAGE` names the running file), and has apt install a new `.deb` behind the system's password prompt (`pkexec`). A snap shows no Updates; snapd does it.
- **A system web view cannot be set in a gpui window on Linux.** What a Mac and Windows show in a web view (an HTML file) is drawn another way there (a renderer of its own, such as Blitz), built for that target alone.

## Checking off a Mac

- **Before the tag:** a MinGW type check of Windows from the Mac (`release-check.sh`), which catches every `cfg` mistake but not a linker one; then `rust.yml` on all six runners. The MSVC target and Linux cannot be checked from a Mac.
- **Before the release is published:** the draft's own installers are installed and opened in VMs on the Mac (Windows 11 and Ubuntu, both ARM), since CI builds and tests the code and nothing else installs what a person downloads (the template's `docs/release.md`, step 6). On Windows the copied installer is first marked as downloaded (`Zone.Identifier`, ZoneId 3), so SmartScreen shows what a person sees.
- **In a VM**, an app started by the VM tool's command runner lives in that command's job and ends with it. Start it the person's way: Explorer, or the desktop's app search.
