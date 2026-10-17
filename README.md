# The Alkaline Software constitution

The standards the studio's products are built to, in two files under
`docs/`:

| File | What it holds |
|---|---|
| [`docs/design.md`](docs/design.md) | everything a person sees: the palette and the accents, the type, the icons, the app icon and how it is drawn, the window and its settings, the menu bar, motion, languages, how a change is checked |
| [`docs/workflow.md`](docs/workflow.md) | how a product is built and released: the version, the changelog, the checks before a tag, CI, signing and notarizing, the disk image, the update inside the app, the dev build, cleaning up, and what went wrong before |

They are distilled from the products that exist, Emaki, Kotoba and
tabflip, which read as one family because each was drawn to these
rules. The bar is the work that wins Awwwards, the Webbys and the FWA:
minimal, calm, intuitive, finished in every state.

## How to use it

**Starting a new product.** Read both files. Then write the product's
own `docs/design.md` and `WORKFLOW.md`: a copy of what applies, in the
product's own terms (its accent, its icon's subject, its window, its
repository and asset names), with what does not apply left out. Copy
the scripts `docs/workflow.md` names from the newest product and change
the names. From then on the product's own docs are its standard, and
nothing in the product points back here: this is a template, not a
dependency.

**With an agent.** Point it at this repository's URL and at the product
you are starting ("follow the constitution to write this product's
`docs/design.md`, then build to it"). The files are written for someone
who can build software, with the reason beside each rule, so an agent
can apply them without being told what each one is for. The product's
`AGENTS.md` then names its own docs, and the agent reads those, not
this.

**Working on a product that exists.** Read the product's own docs; they
are its standard. Come here only for the studio's reasoning behind a
rule, or to see what another product did.

**Bringing something back.** A standard learned in a product that any
product would hit (a palette rule, a motion pattern, a trap in the
toolkit, a release that went wrong) is written here too, in the
product's own words, so the template stays as good as the newest
product. The product's doc is not changed for it.

## The products

| Product | What it is |
|---|---|
| Emaki | a conversation app for agents, the first of the family; the seven accents and the window's layout are its |
| Kotoba | dictation anywhere on a Mac, a menu bar app; the menu bar item, the languages, the update inside the app and the disk image are its |
| tabflip | a window switcher for the Mac, a menu bar app; the list over everything and its motion are its |

Each is Rust and GPUI (Zed's toolkit) with the vendored gpui-component.
A product on another stack draws the same things by the same rules and
names its own helpers.

## Writing here

Plain words, short declarative sentences, the reason after the rule,
numbers in tables, no em dashes. Name the product a rule came from
where it helps, so a reader can go and see it working. `AGENTS.md` says
the rest.
