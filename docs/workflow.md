# Workflow

How an Alkaline Software product is built, run while it is made, and
released: the version, the changelog, the checks before a tag, the tag,
what CI builds, signing and notarizing on the signer's Mac, publishing,
installing, cleaning up, and the pieces a release carries with it (the
icon, the disk image, the update inside the app). Distilled from Emaki,
Kotoba and tabflip, each of which carries a full `WORKFLOW.md` of its
own and does not read this file. A new product takes what applies, in
its own names (its repository, its asset names, its bundle identifier),
leaves out what does not (a Mac-only product has no Windows or Linux
job), and from then on its own `WORKFLOW.md` is its procedure. The
lessons at the end are the products': each guard is there because a
release went wrong without it.

## Rules

- **A release is a tag.** A `v*` tag on the repository starts the
  release workflow, which builds every installer and makes a GitHub
  Release as a **draft**. Nothing is public until the signer publishes
  it, and `releases/latest`, which the app's own update check follows,
  names only published releases. A release left as a draft is not out.
- **The version lives in one file**, the workspace `Cargo.toml`, and the
  tag must agree with it; the workflow refuses otherwise. `Cargo.lock`
  is refreshed after a bump and committed with it, since CI builds with
  `--locked`.
- **The changelog is the release's notes.** `CHANGELOG.md` has one
  `## vX.Y.Z` section per release, newest first, written for the person
  installing the app: what changed for them, one line per change, with
  a short "For developers" part after it only where a change matters to
  someone building the app. The workflow refuses a tag with no section.
- **Everything a tag will be held to is checked before it is pushed**
  (`scripts/release-check.sh`): the version, the changelog section, the
  notary credential, a lock that builds `--locked`, a clean tree, no tag
  of that name yet, the tests, the build, and where the product has one,
  the Windows type check. A tag that fails in CI is a tag redone, with
  the images built and notarized again.
- **The tests run on a clean runner before the tag.** A push to `main`
  runs the test workflow; the tag goes out only once it is green. It is
  the one place a file left out of a commit, or a line that compiles only
  on one platform, is caught before the images are made.
- **Stable asset names.** Each installer is attached under a name that
  never changes (`<App>-mac-arm64.dmg`, `<App>-mac-x64.dmg`,
  `<App>-windows-x64-setup.exe`, `<App>-linux-x64.AppImage`), so
  `releases/latest/download/<name>` stays valid and the app can fetch its
  own update without an API.
- **Nothing personal is in the repository.** The scripts find the
  signing identity in the keychain and the notary credential in a
  keychain profile, or take them from the environment; CI signs ad-hoc
  unless secrets are set.
- **The Mac images on a release are signed with the Developer ID,
  notarized and stapled.** CI's ad-hoc ones are replaced by the signer's
  before the draft is published; a release published with ad-hoc images
  is one macOS refuses after an update.
- **A release ends with nothing compiled left.** `target/` and `dist/`
  are deleted (`scripts/release-clean.sh`), the installed copy is the
  only bundle on the disk, and the dev build hands over to it.
- **One copy of a menu bar app runs at a time.** A copy opened while
  another runs, release or dev, asks it to quit and makes it a few
  seconds on; the one opened last stays. Two would each take the key.

## The repository

What a product's repository carries for this, in the names the
products use:

| Piece | What it is |
|---|---|
| `Cargo.toml` | the workspace; `[workspace.package] version` is the one version |
| `CHANGELOG.md` | one section per release, newest first; the release job reads it |
| `WORKFLOW.md` | the product's own copy of this file, the steps in order |
| `.github/workflows/rust.yml` | the tests and an app build on every push to `main` and every pull request; on every OS the product ships for |
| `.github/workflows/release.yml` | on a `v*` tag: the tag against the version and the changelog, one package job per installer, then a draft release with the installers under their stable names |
| `scripts/dev-app.sh` | the build in `target/debug` as an app bundle of its own, with its own bundle identifier (`<id>.dev`) and entry in Privacy & Security, signed with the developer's identity so permissions outlive a rebuild, in a `.noindex` folder Spotlight passes by |
| `scripts/make-icon.sh`, `scripts/icon/` | the app icon rendered from its page at every size (`docs/design.md`, The app icon) |
| `scripts/dmg/background.py`, `background.png` | the disk image's background (below) |
| `scripts/release-check.sh` | everything a tag will be held to, checked before it is pushed |
| `scripts/release-notes.sh` | one release's section of the changelog, as the release page will show it; fails with no section |
| `scripts/release-mac.sh` | package, sign, lay out the disk image, notarize, staple; one script for a laptop and for CI |
| `scripts/release-publish.sh` | the notarized images over CI's, checked from the outside, and the draft published |
| `scripts/release-clean.sh` | the end of a release: `target/` and `dist/` gone, one bundle left, the dev build handed over |
| `crates/<app>-core/src/update.rs` | the newest version, the installer, and putting it in place (below) |

## Each release

1. **Bump the version** in `Cargo.toml`; `cargo check` so the lock
   follows.
2. **Write the changelog section.** `scripts/release-notes.sh X.Y.Z`
   prints what the release page will show.
3. **Commit, then `scripts/release-check.sh X.Y.Z`.** Do not tag until
   it says "ready to tag".
4. **Push `main`, wait for the test workflow, then tag:**
   `git tag -a vX.Y.Z -m "<App> vX.Y.Z"` and
   `git push origin refs/tags/vX.Y.Z` (by its full name: a branch of the
   tag's name made the short form ambiguous once). The tag starts the
   release workflow, about twenty minutes for a cold GPUI build.
5. **Build, sign and notarize the Mac images on the signer's Mac**,
   from the tagged commit, while CI runs: a release build for each
   architecture, then `scripts/release-mac.sh --no-build --notarize`
   for each. Each run ends with `built dist/<name> (X.Y.Z, <arch>,
   signed, notarized)`.
6. **Publish:** `scripts/release-publish.sh X.Y.Z`, which refuses unless
   the release workflow for the tag is green and both images are
   notarized, stapled and newer than the tag's commit; then uploads them
   over CI's, downloads them back and compares checksums, and takes the
   draft public as the latest release. The release is not done until it
   prints so.
7. **Install the release build** from the image into `/Applications`.
   A release build is its own entry in Privacy & Security, apart from
   the dev build's; a later release signed with the same Developer ID
   keeps what it was given.
8. **Delete the compiled files:** `scripts/release-clean.sh`, every
   time, without being asked, once step 6 printed its line and the
   installed copy opens.

Redoing a release under the same tag: delete the release and the tag on
both sides, cancel a run still going, and repeat from step 3 with the
new commit; the images are rebuilt from it and published again.

## The disk image

The Mac installer is a disk image the person opens and drags from, laid
out so that the first sight says what to do.

- **The window** is 660 by 420 points: the app icon at the left (165,
  260), the Applications link at the right (495, 260), both at 128
  points, over a background that says "Drag <App> to Applications" with
  an arrow between them. The volume is named `<App> vX.Y.Z`.
- **The background** is `scripts/dmg/background.png`, 1320 by 840 at
  144 dpi (the window at 2x), drawn by `scripts/dmg/background.py` with
  Pillow: the words in LXGW WenKai Medium, auto-sized to 85% of the
  width, in the ink on the app's page colour (Kotoba's cream, tabflip's
  near white), and the arrow in the product's accent, its shaft ending
  at the arrowhead's base so it does not poke through. The PNG is
  committed; the script runs only when the picture changes (`uv run
  --with pillow python3 scripts/dmg/background.py`).
- **The layout** is Finder's. `release-mac.sh` makes a read-write image,
  mounts it, and has Finder (by AppleScript) set the window's bounds,
  icon view, icon size, background and the two positions, then converts
  it to a compressed read-only image. Only Finder writes a `.DS_Store`
  Finder honours, so the layout is not generated offline. Housekeeping
  items (`.background`, `.fseventsd`, `.Trashes`, `.DS_Store`) are placed
  far outside the window. This works on the GitHub macOS runners too.
- **The bundle** inside is signed with the Developer ID and the hardened
  runtime (`--options runtime --timestamp`, `--deep` for helpers), the
  image signed after it, then notarized and stapled.

## The update inside the app

A product keeps itself current from the release's stable asset names,
with no API and no server of the studio's own.

- **The newest version** is the tag `releases/latest` redirects to
  (`version_from_location`), asked once a day while Check automatically
  is on and whenever the button is pressed. Drafts are not seen. The
  check runs off the main thread; a product with no HTTPS client in it
  (tabflip) asks through `curl`, which every Mac has.
- **One button in one place.** Updates shows the version, what a check
  found, and a button that is "Check for updates" until a check finds
  something and "Update to x" after, filled with the accent, with
  "Release notes" under the version then. Nothing is fetched or
  installed without that press. The rail's Updates wears a dot while a
  newer version waits.
- **The download** goes to the app's cache folder under its stable
  asset name, reporting bytes so far and the total when known, which the
  row says as "Downloading… {p}%"; a download that ends short of the
  total is discarded.
- **Installing on a Mac:** the image is mounted at a mount point of the
  app's own, `<App>.app` copied out beside the running bundle with
  `ditto` (which keeps the signature and the attributes), the running
  bundle moved aside and the copy renamed into its place (two renames,
  the first undone if the second fails), the old one deleted; then the
  app quits, and a shell of its own, waiting on the old process's pid
  and not on a length of time, opens the new bundle once it has ended.
  Never two at once: opened beside the old one, the new app was a second
  app to the Dock. The old process keeps running from its unlinked files
  until it quits.
- **Elsewhere:** on Windows the installer is started and replaces the
  files once the app has quit; on Linux an AppImage is replaced in place
  (`APPIMAGE` names the running file) and started again, and a `.deb`
  install is sent to the release page.
- **What can go wrong is said in the row**, in plain words ("The update
  did not go through: …"), and the button comes back.
- **A trial short of the relaunch:** a command (`<app> --try-update
  <bundle>`, Kotoba's CLI) does the download and the swap on a throwaway
  copy of the bundle and opens nothing, so the steps can be checked
  without replacing the installed app.

## Signing in CI

The release workflow signs and notarizes on the runner when five
repository secrets exist (the Developer ID certificate as a base64
`.p12` and its passphrase, the Apple ID, an app-specific password and
the team ID); step 5 then disappears and step 6 is only taking the
draft public. Until then CI's images are ad-hoc and the signer's replace
them.

## When something fails

Every failure met while cutting a release gets a line in the product's
`WORKFLOW.md` once it is solved, with what would have caught it, and the
steps are changed so it is caught before the tag next time. The ones met
so far, worth carrying into a new product:

- **A tag pushed with the tests still running** went red on another OS
  (a file name's trailing dot, ignored on Windows): wait for the test
  workflow before tagging.
- **A line that compiles only on one platform** (something gated for
  Unix used everywhere): the Windows type check in `release-check.sh`.
- **`hdiutil create` fails with "Resource busy"** on a machine that has
  just built, a GitHub runner most of all: try it five times, and
  without `-quiet`, which hid why a job stopped.
- **Finder "AppleEvent timed out (-1712)"** while laying the image out:
  macOS asked whether the terminal may control Finder and nobody
  answered. Allow it under Privacy & Security, Automation, and run
  again.
- **The notary credential cannot be reached from an agent's session**
  while the same commands work in the person's own terminal: run step 5
  there. `release-check.sh` asks the notary service before anything is
  built, so it is known before the tag.
- **"src refspec vX.Y.Z matches more than one"**: a branch has the tag's
  name; push the tag by its full name and name a release's branch apart
  from its tag.
- **Published with ad-hoc images** because step 6 was skipped: the app
  updated itself from them and macOS refused it. `release-publish.sh`
  refuses unless the images are notarized.
- **A release left as a draft**: the app's update check finds nothing,
  since `releases/latest` names only published releases (tabflip's first
  three). Step 6 is the release.
- **Two bundles on the disk** (`dist/`, a scratch folder, an old dev
  build in `target/debug`): Launchpad and Spotlight list each. The dev
  bundle lives in a `.noindex` folder, and step 8 removes the rest.
- **A grant bound to an old signature:** macOS binds an Accessibility
  grant to the app's code signature. An entry made for an earlier copy
  signed otherwise (a build before the Developer ID releases) shows as
  on in System Settings and counts for nothing for the new copy, and
  switching it off and on changes nothing; only removing it and
  granting again works (tabflip v0.2.2). Allow does that itself:
  `tccutil reset Accessibility <bundle id>` (no `sudo`; the bundle must
  be on disk) before the system's question, so one press and one switch
  is all it takes whether or not an older copy was ever allowed.
- **A grant read once:** `AXIsProcessTrusted` answers a running process
  with its launch-time value, so a permission given while the app runs
  went unseen until a relaunch (tabflip v0.2.0). Whether a permission is
  given is whether the thing it allows can be done (the tap made), tried
  after the person's press and at each change the system reports.
