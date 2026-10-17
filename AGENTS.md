# AGENTS.md

## What this is

The constitution of **Alkaline Software**: the standards the studio's
products are built to, distilled from the products that exist, kept in
one place so the next product starts where the last one got to.
Canonical source: https://github.com/alkaline-software/constitution.

It is a template, not a dependency. A new product is seeded from it;
once the product has its own docs, those docs are the product's whole
standard and nothing in the product points back here. Emaki, Kotoba
and emaki.lol each carry a full `docs/design.md` of their own and do
not read this repo.

## The files

Every standard lives under `docs/`. This file is the only one at the
root.

| Starting | Read |
|---|---|
| Anything a person sees: a window, a page, a control, a colour, an icon, a menu bar item, motion, a settings panel | `docs/design.md` |

More will come (how a release is cut, how a repo is laid out, how a
product writes its docs), each as a file under `docs/` with a row
here.

## Starting a new product

- **Read the file the table names, then write the product's own.** The
  product's `docs/design.md` is a copy of what applies, in the
  product's own terms: its accent, its icon's subject, its window's
  size, the parts it has, the helpers in its code that draw each
  pattern. What does not apply is left out. What is left is the
  product's, to change as the product needs.
- **Nothing in the product refers here.** Its `AGENTS.md` names its own
  `docs/`, as Emaki's and Kotoba's do. A member who wants the studio's
  reasoning behind a rule comes to this repo by its URL.
- **Bring back what the next product would want.** A standard learned
  in a product that any product would hit (a palette rule, a motion
  pattern, a trap in the toolkit) is written here too, in the product's
  own words, so the template stays as good as the newest product. The
  product's doc is not changed for it.

## Writing things down

- Write what is so now and why, in plain words, to someone who can
  build software. The reason stays with the rule: a template that only
  says what to do is copied without thought.
- Name the product a rule came from where it helps (Emaki's seven
  accents, Kotoba's menu bar), so a reader can go and see it working.
- No em dashes. Keep the voice the files have: short declarative
  sentences, the reason after the rule, numbers in tables.
