# The Alkaline Software constitution

The standards the studio's products are built to: five documents under `docs/`, and `template/`, a product that holds them as code.

| File | What it holds |
|---|---|
| [`docs/design.md`](docs/design.md) | everything a person sees: the palette, the accents and code's inks, the faces, the icons (Tabler, Catppuccin, and drawing one neither has), the app icon and how it is made, the window, its settings, controls and scrollers, the menu bar, motion and frames, languages, how a change is checked |
| [`docs/structure.md`](docs/structure.md) | how a product is put together: Rust and GPUI, the core and app crates, the vendored toolkit, the repository's folders, what each file of the app does, probing from a script |
| [`docs/platforms.md`](docs/platforms.md) | macOS, Windows and Linux, each on Intel and ARM: what each system needs, what is gated, the installers, checking off a Mac |
| [`docs/docs.md`](docs/docs.md) | how a product keeps its words: `AGENTS.md`, a doc a part, the changelog, how a doc is written |
| [`docs/workflow.md`](docs/workflow.md) | how a product is built and released: the dev build, the version, the checks before a tag, CI, signing and notarizing, trying the installers in VMs, the stores, the update inside the app, cleaning up, and what went wrong before |
| [`template/`](template) | **Starter**, a Rust and GPUI app for the three systems with every pattern above in working code: the window, the settings window, the buttons, the choices, the switch, the fold, the menu, the scroll router, the window's clock, the accents, eight languages, the icons and the app icon's pipeline, the release scripts and CI |

The bar is the work that wins Awwwards, the Webbys and the FWA: minimal, calm, intuitive, finished in every state.

## How to use it

**Starting a new product.**

```
scripts/new-product.sh ../<folder> <Name> <bundle id> <owner/repo>
```

copies `template/` to a folder of its own and renames every name Starter has. Then: replace the sample page (`Shell::render_page` in `shell.rs`), draw the product's icon in `scripts/icon/icon.html` and run `scripts/make-icon.sh`, and write the product's `docs/design.md` and `docs/platform.md` from this repository's, in the product's own terms (its accent, its icon's subject, its window, its parts), with what does not apply left out. From then on the product's own docs are its standard, and nothing in the product points back here: this is a template, not a dependency.

**With an agent.** Point it at this repository and at the folder to start in ("make a product called X from the constitution, then build Y in it"). The documents are written for someone who can build software, with the reason beside each rule, so an agent applies them without being told what each is for.

**Working on a product that exists.** Read the product's own docs; they are its standard. Come here for the studio's reasoning behind a rule.

**Bringing something back.** A standard learned in a product that any product would hit (a palette rule, a motion pattern, a trap in the toolkit, a release that went wrong) is written here too, in the documents and, where it is code, in `template/`, so the template stays as good as the newest product.

## Checking the template

```
cd template
cargo test -p starter-core -p starter-app
cargo build -p starter-app && STARTER_HOME=/tmp/starter ./target/debug/Starter
```

## Writing here

Plain words, short declarative sentences, the reason after the rule, numbers in tables, no em dashes, no hard wraps, American spelling. Nothing here names another repository: what a product learned is written here in general terms, and what it built is copied into `template/`. `AGENTS.md` says the rest.
