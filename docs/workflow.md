# Workflow

How an Alkaline Software product is built, run while it is made, and released on three systems: the rules, the assets a release carries, the scripts, the dev build, the disk image, the update inside the app, signing, the stores, and what went wrong before. The steps themselves, command by command, are the template's `docs/release.md`, which a new product carries as its own, and the scripts are in `template/scripts/`. The lessons at the end are why each guard is there: a release went wrong without it.

## Rules

- **A release is a tag.** A `v*` tag on the repository starts the release workflow, which builds every installer and makes a GitHub Release as a **draft**. Nothing is public until the signer publishes it, and `releases/latest`, which the app's own update check follows, names only published releases. A release left as a draft is not out.
- **The version lives in one file**, the workspace `Cargo.toml`, and the tag must agree with it; the workflow refuses otherwise. `Cargo.lock` is refreshed after a bump and committed with it, since CI builds with `--locked`.
- **The changelog is the release's notes** (`docs/docs.md`, The changelog). The workflow refuses a tag with no section.
- **Everything a tag will be held to is checked before it is pushed** (`scripts/release-check.sh`): the version, the changelog section, the notary credential, a lock that builds `--locked`, a clean tree, no tag of that name yet, the tests (once more with Windows' one-megabyte stack), the build, and the Windows type check. A tag that fails in CI is a tag redone, with the Mac images built and notarized again.
- **The tests run on a clean runner on every system before the tag.** A push to `main` runs the test workflow on all six runners; the tag goes out only once it is green. It is the only place the tests run on Windows and Linux.
- **The installers are tried on the other systems before the release is public**, in VMs, from the draft itself (step 6). CI builds and tests the code; nothing else installs what a person downloads.
- **Stable asset names.** Each installer is attached under a name that never changes, so `releases/latest/download/<name>` stays valid and the app fetches its own update without an API.
- **Nothing personal is in the repository.** The scripts find the signing identity in the keychain and the notary credential in a keychain profile, or take them from the environment; CI signs ad-hoc unless secrets are set.
- **The Mac images on a release are signed with the Developer ID, notarized and stapled.** CI's ad-hoc ones are replaced by the signer's before the draft is published; a release published with ad-hoc images is one macOS refuses after an update.
- **A release ends with nothing compiled left.** `target/` and `dist/` are deleted (`scripts/release-clean.sh`), the installed copy is the only bundle on the disk, and the dev build hands over to it.
- **One copy of a menu bar app runs at a time.** A copy opened while another runs, release or dev, asks it to quit and makes it a few seconds on; the one opened last stays. Two would each take the key.

## The assets

| asset | built by | signed |
|---|---|---|
| `<App>-mac-arm64.dmg`, `<App>-mac-x64.dmg` | CI, then replaced from the signer's Mac | Developer ID, notarized, stapled |
| `<App>-windows-x64-setup.exe`, `<App>-windows-arm64-setup.exe` | CI (Velopack) | Azure Artifact Signing once its secrets are set |
| `<pack id>-<version>-win-<arch>-full.nupkg`, `releases.win-<arch>.json` | CI (Velopack) | the package and the feed a Windows copy updates itself from |
| `<App>-linux-x64.AppImage`, `<App>-linux-arm64.AppImage` | CI | n/a |
| `<App>-linux-x64.deb`, `<App>-linux-arm64.deb` | CI | n/a |

Every one is needed: each installer is a first install and the update for one kind of copy (`update.rs` picks by system, chip and how the copy was installed), and the `.nupkg` and its feed are how a Velopack copy updates. The Windows feed and package carry the version and are found through the feed, whose own name is stable. The Microsoft Store's `.msix` and the snaps are not on the release: CI keeps them with the run for their stores.

## The repository

What a product's repository carries for this, as the template has it:

| Piece | What it is |
|---|---|
| `Cargo.toml` | the workspace; `[workspace.package] version` is the one version |
| `CHANGELOG.md` | one section per release, newest first; the release job reads it |
| `docs/release.md` | the steps in order, with the commands, and the product's own lessons |
| `.github/workflows/rust.yml` | the tests and an app build on every push to `main` and every pull request, on `macos-latest`, `windows-latest`, `windows-11-arm`, `ubuntu-24.04` and `ubuntu-24.04-arm`, with `Swatinem/rust-cache` a system; the core's home folders pointed at the runner's temp |
| `.github/workflows/release.yml` | on a `v*` tag: the tag against the version and the changelog, one package job per installer, then a draft release with the installers under their stable names |
| `scripts/dev-app.sh` | the build in `target/debug` as an app bundle of its own (`docs/platforms.md`, macOS) |
| `scripts/relaunch.sh` | quit the dev build and start the new one once the agent's turn is over (`--installed` starts the one in `/Applications`) |
| `scripts/make-icon.sh`, `scripts/icon/` | the app icon rendered from its page at every size (`docs/design.md`, The app icon) |
| `scripts/dmg/background.py`, `background.png` | the disk image's background (below) |
| `scripts/splash/` | the Windows installer's splash |
| `scripts/windows-msix.ps1`, `scripts/windows-store-assets.py` | the Microsoft Store's package and its tiles |
| `scripts/linux-snap.sh` | the snap, from a Linux build |
| `scripts/release-check.sh` | everything a tag will be held to, checked before it is pushed |
| `scripts/release-notes.sh` | one release's section of the changelog, as the release page will show it; fails with no section |
| `scripts/release-mac.sh` | package, sign, lay out the disk image, notarize, staple; one script for a laptop and for CI |
| `scripts/release-publish.sh` | the notarized images over CI's, checked from the outside, and the draft published |
| `scripts/release-clean.sh` | the end of a release: `target/` and `dist/` gone, one bundle left, the dev build handed over |
| `crates/<app>-core/src/update.rs` | the newest version, the installer, and putting it in place (below) |

## The dev build

- **One dev build at a time.** Two copies share the app's home folder and both write what the window remembers. The installed copy beside it is expected; a second dev build is a mistake. A probe copy has a home of its own (`docs/structure.md`, Probing).
- **After a change to the app, build it and relaunch it** (`cargo build -p <app>-app`, then `scripts/relaunch.sh`) as the last command of the task, without being asked. The script returns at once and does the swap some seconds after the agent's turn is over, so the person can read the reply first. A change to documents alone needs no relaunch.
- **An agent working inside the app is the app's child.** Quitting the app ends the agent's session, so nothing quits it by hand (`pkill`); the relaunch script waits while the session is busy.

## Each release

The template's `docs/release.md` has each step's commands. In short, and why each is there:

1. **Bump the version** in `Cargo.toml` and refresh the lock, since CI builds `--locked`.
2. **Write the changelog section**, which is the release's notes.
3. **Keep the display on and run `release-check.sh`.** The notary credential is out of reach while the screen is locked, and the check holds the commit to everything the tag will be held to, so a tag is not redone for a mistake a minute's check finds.
4. **Push, wait for the tests on all six runners, then tag.** CI is the only place the tests run on Windows and Linux.
5. **Build, sign and notarize the Mac images on the signer's Mac**, from the tagged commit, while CI builds the rest.
6. **Install and run the draft's installers on Windows and Linux**, in VMs: CI builds and tests the code, and nothing else installs what a person downloads.
7. **Publish with `release-publish.sh`**, which puts the notarized images over CI's ad-hoc ones and checks them from the outside before the draft goes public.
8. **Install the release build** beside the old bundle and rename it into place, never copied over a running app.
9. **Delete the compiled files** with `release-clean.sh`, every time, without being asked.

Redoing a release under the same tag deletes the release and the tag on both sides and repeats from step 3 with the new commit.

## The disk image

The Mac installer is a disk image the person opens and drags from, laid out so that the first sight says what to do.

- **The window** is 660 by 420 points: the app icon at the left (165, 260), the Applications link at the right (495, 260), both at 128 points, over a background that says "Drag <App> to Applications" with an arrow between them. The volume is named `<App> vX.Y.Z`.
- **The background** is `scripts/dmg/background.png`, 1320 by 840 at 144 dpi (the window at 2x), drawn by `scripts/dmg/background.py` with Pillow: the words in LXGW WenKai Medium, auto-sized to 85% of the width, in the ink on the app's page color, and the arrow in the product's accent, its shaft ending at the arrowhead's base so it does not poke through. The PNG is committed; the script runs only when the picture changes.
- **The layout** is Finder's. `release-mac.sh` makes a read-write image, mounts it, and has Finder (by AppleScript) set the window's bounds, icon view, icon size, background and the two positions, then converts it to a compressed read-only image. Only Finder writes a `.DS_Store` Finder honors, so the layout is not generated offline. Housekeeping items (`.background`, `.fseventsd`, `.Trashes`, `.DS_Store`) are placed far outside the window. This works on the GitHub macOS runners too.
- **The bundle** inside is signed with the Developer ID and the hardened runtime (`--options runtime --timestamp`, `--deep` for helpers), the image signed after it, then notarized and stapled.

## The update inside the app

A product keeps itself current from the release's stable asset names, with no API and no server of the studio's own.

- **The newest version** is the tag `releases/latest` redirects to, asked once a day while Check automatically is on and whenever the button is pressed. Drafts are not seen. The check runs off the main thread; a product with no HTTPS client in it asks through `curl`, which every Mac has.
- **One button in one place.** Updates shows what a check found, and a button that is "Check for updates" until a check finds something and "Update to x" after, filled with the accent, with "Release notes" then. Nothing is fetched or installed without that press. The rail's section wears a dot while a newer version waits.
- **The download** goes to the app's cache folder under its stable asset name, reporting bytes so far and the total when known, which the row says as "Downloading… {p}%"; a download that ends short of the total is discarded.
- **On a Mac:** the image is mounted at a mount point of the app's own, `<App>.app` copied out beside the running bundle with `ditto` (which keeps the signature and the attributes), the running bundle moved aside and the copy renamed into its place (two renames, the first undone if the second fails), the old one deleted; then the app quits, and a shell of its own, waiting on the old process's pid and not on a length of time, opens the new bundle once it has ended. Never two at once: opened beside the old one, the new app was a second app to the Dock.
- **On Windows:** a Velopack copy reads its architecture's feed (`releases.win-<arch>.json` from `releases/latest/download`), fetches the package it names and starts Velopack's updater, which waits for the app to quit, puts the package in place and opens it. A copy from an older installer fetches the new `Setup.exe` and starts it.
- **On Linux:** an AppImage is replaced in place (`APPIMAGE` names the running file) and started again; a `.deb` install has apt install the new `.deb` behind the system's password prompt (`pkexec`), then starts the new binary.
- **A store's copy never updates itself** (the Microsoft Store's, known by its `WindowsApps` folder; a snap, by `SNAP`), and shows no Updates.
- **What can go wrong is said in the row**, in plain words ("The update did not go through: …"), and the button comes back.
- **A trial short of the relaunch:** the core's CLI (`<app>-core --try-update <version> <copy of the bundle>`) does the download and the swap on a throwaway copy, and on Windows `<APP>_UPDATE_FEED` names a feed of one's own, so the steps can be checked without replacing the installed app.

## Signing in CI

The release workflow signs and notarizes the Mac images on the runner when five repository secrets exist (the Developer ID certificate as a base64 `.p12` and its passphrase, the Apple ID, an app-specific password and the team ID); step 5 then disappears and step 7 is only taking the draft public. Until then CI's images are ad-hoc and the signer's replace them.

## Signing on Windows

SmartScreen stops an unsigned installer with "Windows protected your PC". `release.yml` has `vpk` sign the app, Velopack's programs and `Setup.exe` with Azure Artifact Signing when its secrets exist, and leaves them unsigned otherwise; the template's `docs/release.md` says how to set it up. A new certificate still meets SmartScreen's warning until downloads build its reputation; the same identity across releases keeps what was built. The client secret expires within two years.

## The stores

- **The Microsoft Store** signs what it sells, so its copy never meets SmartScreen, and updates it. Every release run keeps an `.msix` an architecture once the Store's identity is set; it is submitted by hand. It asks for `runFullTrust` and `unvirtualizedResources`, so a shell or a tool started in the app installs into the real AppData.
- **The Snap Store** installs with no warning, where App Center warns "Potentially unsafe" about every `.deb` opened from a file, whoever made it, and snapd keeps it current. Classic confinement is asked for on the snapcraft forum (days to weeks), since a product that runs the person's shell or reads their files in their home cannot be strict. Once its credentials are a secret, every release run uploads both snaps.

## When something fails

Every failure met while cutting a release gets a line in the product's `docs/release.md` once it is solved, with what would have caught it, and the steps are changed so it is caught before the tag next time. The ones met so far, worth carrying into a new product:

- **A tag pushed with the tests still running** went red on another system (a file name's trailing dot, ignored on Windows): wait for the test workflow before tagging.
- **A line that compiles only on one platform** (something gated for Unix used everywhere): the Windows type check in `release-check.sh`.
- **A test passes on a Mac and fails on Windows** on a line end (`core.autocrlf`), a drive letter read as a host, or a stack overflow (one megabyte to a thread): `docs/platforms.md`, Windows.
- **The window opened lower at each launch on Windows**, saved from the wrong bounds: step 6 closes and opens the window twice.
- **`hdiutil create` fails with "Resource busy"** on a machine that has just built, a GitHub runner most of all: try it five times, and without `-quiet`, which hid why a job stopped. If it still fails, re-run only that job (`gh run rerun <run-id> --failed`).
- **Finder "AppleEvent timed out (-1712)"** while laying the image out: macOS asked whether the terminal may control Finder and nobody answered. Allow it under Privacy & Security, Automation, and run again.
- **"No Keychain password item found for profile"** while the profile is in place: the Mac's screen is locked. The notary profile is in a keychain that opens only while the screen is unlocked, and a Mac that locks when the display sleeps loses it overnight, in an agent's session or anyone's, while builds and CI go on. It is not the sandbox. Keep the display on (step 3) and notarize while the person is at the Mac; `release-check.sh` asks the notary service before anything is built, so it is known before the tag.
- **"src refspec vX.Y.Z matches more than one"**: a branch has the tag's name; push the tag by its full name and name a release's branch apart from its tag.
- **Published with ad-hoc images** because step 7 was skipped: the app updated itself from them and macOS refused it. `release-publish.sh` refuses unless the images are notarized.
- **A release left as a draft**: the app's update check finds nothing, since `releases/latest` names only published releases. Step 7 is the release.
- **Two bundles on the disk** (`dist/`, a scratch folder, an old dev build in `target/debug`): Launchpad and Spotlight list each. The dev bundle lives in a `.noindex` folder, and step 9 removes the rest.
- **The release-clean script did not see the running dev build**: `pgrep` leaves out its own ancestors, and the app is an ancestor of the session that runs the script. Find it with `ps`.
- **"Node.js NN is deprecated" on a run**: an action is a major version behind; move `rust.yml` and `release.yml` together.
- **A grant bound to an old signature:** macOS binds an Accessibility grant to the app's code signature. An entry made for an earlier copy signed otherwise shows as on in System Settings and counts for nothing for the new copy, and switching it off and on changes nothing; only removing it and granting again works. Allow does that itself: `tccutil reset Accessibility <bundle id>` (no `sudo`; the bundle must be on disk) before the system's question.
- **A grant read once:** `AXIsProcessTrusted` answers a running process with its launch-time value, so a permission given while the app runs went unseen until a relaunch. Whether a permission is given is whether the thing it allows can be done, tried after the person's press and at each change the system reports.
