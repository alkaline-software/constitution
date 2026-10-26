# AGENTS.md

## What this is

The constitution of **Alkaline Software**: the standards the studio's products are built to, kept in one place so the next product starts where the last one got to. Five documents under `docs/` say what the standard is and why; `template/` is the standard as a product that builds, tests and runs.

It is a template, not a dependency. A new product is made from `template/` by `scripts/new-product.sh`; once the product has its own docs, those docs are its whole standard and nothing in the product points back here.

## Rules

- **It stands alone.** Nothing here names or points to another repository, product or folder outside this one. A rule says what is so and why in general terms; a pattern that needs code has it in `template/`. A product's history (its versions, its names) does not appear.
- **The documents and the template agree.** A helper, a file or a variable a document names exists in `template/` under that name. A change to one is a change to both.
- **The template builds and its tests pass** (`cargo test -p starter-core -p starter-app` in `template/`), and a product made by `scripts/new-product.sh` does too. Check both after a change to either.
- **The template is looked at, not only built.** A change to what it draws is shot in both appearances and in a long language (`template/docs/probing.md`).

## The files

| Starting | Read |
|---|---|
| Anything a person sees: a window, a page, a control, a button, a color, the faces, an icon (Tabler's, Catppuccin's, one drawn by hand), the app icon, a menu bar item, motion, scrollers, a settings window, the languages | `docs/design.md` |
| How a product is put together: Rust and GPUI, the two crates, the vendored toolkit, the repository's folders, the files of the app crate, probing | `docs/structure.md` |
| macOS, Windows and Linux on Intel and ARM: what each needs, what is gated, the installers, checking off a Mac | `docs/platforms.md` |
| How a product keeps its words: `AGENTS.md`, a part's doc, the changelog, how a doc is written | `docs/docs.md` |
| How a product is built and released: the dev build, the version, the checks, the tag, CI, signing and notarizing, trying the installers in VMs, the stores, the update inside the app, cleaning up | `docs/workflow.md`, then `template/docs/release.md` |
| The template's code | `template/AGENTS.md` |

## Bringing something back

A standard a product learned that any product would hit (a palette rule, a motion pattern, a trap in the toolkit, a release that went wrong) is written here: the rule and its reason in the document it belongs to, in general terms, and the code in `template/` where it is code. The product's own doc is not changed for it.

## Writing things down

- Write what is so now and why, in plain words, to someone who can build software. The reason stays with the rule: a template that only says what to do is copied without thought.
- The rules of `docs/docs.md`, Writing, hold here: no hard wraps, American spelling, no em dashes, short declarative sentences, the reason after the rule, numbers in tables.
