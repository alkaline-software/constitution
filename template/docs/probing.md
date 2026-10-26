# Probing

How to look at a state of the window from a script, without the person's hands.

## Rules

- Probe with a second copy, never the copy the person is using: `STARTER_HOME` at a scratch folder.
- Say what was checked and what was not. A synthetic click does not reach a click handler in a background window, so a picture of the state a click leads to does not show the click works.

## A copy

```
cargo build -p starter-app
STARTER_HOME=<scratch>/home STARTER_GO=x STARTER_SHOT=<file.png> ./target/debug/Starter
```

| Variable | What it does |
|---|---|
| `STARTER_HOME` | the copy's own settings and state |
| `STARTER_APPEARANCE` | `light` or `dark`, for this launch only; never written back |
| `STARTER_LANG` | a language's code (`de`, `zh-Hans`), over the system's |
| `STARTER_SETTINGS` | open the settings on that section (`appearance`, `general`, `about`) |
| `STARTER_GO` | steps, `;` between: `fold`, `sidebar`, `work`, `menu`; any value keeps the window over the others |
| `STARTER_SHOT` | a picture of the window (the settings when they are open) after it is drawn, then quit; `STARTER_SHOT_FRAME=1` keeps the system's shadow and rim |

A process may take a picture of its own windows without Screen Recording access, which a terminal running a probe usually lacks.

## The core from a terminal

```
cargo run -p starter-core -- paths | config | update
cargo run -p starter-core -- --try-update <version> <copy of Starter.app>
```
