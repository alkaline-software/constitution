# Docs

How a product's repository keeps its words: which files there are, what each is for, how a doc is laid out, and how it is written. Most of a product's sessions are an agent's, and its docs are what keep a fixed bug from coming back. The rule behind all of it: a trap is written where the next person to touch that code will read it, before they touch it.

## Rules

- **Each document has one job.** `AGENTS.md` is what the product is, the rules every session follows, and where to read next. `docs/<part>.md` is how one part works and why, with its traps. `docs/release.md` is how to cut a release. `CHANGELOG.md` is what each release changed for the person installing it. `docs/probing.md` is how to look at the window from a script: the variables, the steps, and the rules a probe copy keeps (`docs/structure.md`, Probing). `README.md` is a short introduction for GitHub that sends the reader to the product's site. Nothing else is kept as a file: a plan, a roadmap or a list of tasks is out of date within weeks and then misleads whoever reads it.
- **Read the doc before the code.** `AGENTS.md` has a table of what is about to be touched and the doc to read first, and the doc is read even for a small change, since the traps are there and nowhere else. A subagent handed work on a file is told to read its doc too.
- **A fact goes where its code is.** How a feature works, a trap, a fact learned about the toolkit or another program goes in that part's doc, under `## Rules` when breaking it loses data or brings back a bug. `AGENTS.md` gets a line only for a rule every session follows whatever it works on, or a row in its table. Keep it under 300 lines.
- **What is so now, and why.** Not the history of how it got there. An approach tried and dropped gets one sentence, and only where someone might try it again ("Tried and dropped: …"). What was checked and how belongs in the reply to the person, not in a doc.
- **Not what the code says by itself:** a constant's value, a pixel size, a list of function names. Name the function so a reader can find it; leave its body to say what it does.
- **No private memory in place of the doc.** A lesson an agent learns goes in the repository's docs, where every agent and person reads it.

## AGENTS.md

In this order:

1. **What this is**, a paragraph, then the one idea the product stands on, with why.
2. **The standard**: the design bar and the rules of it every change meets unasked (`docs/design.md`).
3. **Rules that hold everywhere**, one line each with the doc that has the story in brackets. Each is there because breaking it lost data or brought back a fixed bug.
4. **Before you edit, read**: the table of file or part, and doc.
5. **Layout**: the tree of the repository with a few words a file, so a reader knows where a thing is before they search.
6. **Running it**: the dev build, one at a time, how it is relaunched after a change, how a release is cleaned up.
7. **Writing things down**: this file's rules, in the product's words.
8. **Testing**: the commands, and what isolates the tests from the person's data.

## A part's doc

- The first line under the title says what it is about, by file: "About `look.rs`, `fonts.rs`, `assets.rs`, `assets/`, `themes/<app>.json`."
- `## Rules` next: what must not be broken, each with its reason in the same item.
- Then a section a piece of the part, as long as the piece needs.
- No section on probing. How the window is driven from a script is one file for the product, `docs/probing.md`, since the variables and steps are one set the whole window answers to; a part's doc points to it where a state of the part is to be looked at (Checking in `docs/design.md`, a language in the window's doc).
- A change that crosses two parts reads both docs and writes in the one whose code changed.

## The changelog

`## vX.Y.Z` a release, newest first, written for the person installing the app: what changed for them, not which files moved. A numbered list, a topic in bold and then one sentence, small fixes gathered under **Fixes**, ten items at most. The release job uses the section as the release's notes and refuses a tag without one.

## Writing

- Plain words, short declarative sentences, the reason after the rule, numbers in tables. Few adverbs and qualifiers: a word that adds nothing goes.
- **No hard wraps.** A paragraph, a list item or a comment's paragraph is one line however long; a line break ends it. The editor wraps; the file does not. This holds in every Markdown file and in prose comments.
- **American spelling** (color, center, license, gray), whatever another repository uses.
- **No em dashes.** Commas, periods, parentheses, semicolons or colons instead.
- **The window's words are few** (`docs/design.md`, Words are few), and so are a doc's: say a thing once, in the place it belongs, and point to it from elsewhere.
- **Name the place, not a line number**: `docs/design.md`, Motion; `widgets.rs`, `choice_pill`. Line numbers move.
- **A commit message says what changed and why, for the person reading the history,** in the same plain voice. A commit is made only when the person asks for one.
