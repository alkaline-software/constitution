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
| `STARTER_GO` | steps, `;` between: `fold`, `sidebar`, `work`, `menu`, `updatefound` (a newer version waits: the gear's and About's dots; nothing is fetched); any value keeps the window over the others |
| `STARTER_SHOT` | a picture of the window (the settings when they are open) after it is drawn, then quit; `STARTER_SHOT_FRAME=1` keeps the system's shadow and rim |

A process may take a picture of its own windows without Screen Recording access, which a terminal running a probe usually lacks.

## Measuring

A speed-up starts from a number and ends with another. The stopwatch goes in a copy and comes out when it is done; none is kept.

- **At rest:** `top -pid <pid> -stats cpu,idlew,mem`, with the window closed and open. A product at rest is at about 0% with a few wakeups a second. What is left to find is in what it does at work: go through each path, and read what runs on a timer.
- **A frame:** the time from the top of `render` to a `canvas` painted last. A hover draws the window whole, so a frame asked for by a timer costs what a hover costs. A frame has about 16 ms.
- **A wait outside a frame:** a timer of 5 ms on the main thread that prints any gap over 30 ms. It finds a lock, a sound or a file that no frame shows.
- **A pointer with no hands:** a sweep through `window.dispatch_event` (`PlatformInput::MouseMove`, and a press and a release on each section), so the person's cursor stays where it is.
- **A wait that comes and goes:** the same thing done twice running, and done after a pause of several seconds. A system that has gone quiet takes a moment to wake.
- **A rate:** the app's CPU time and the window server's over the same seconds, doing nothing, as it was, and as it is. Frames cost the window server too, and with no baseline to take off, a saving does not show.
- **Work on the GPU:** a call that hands it work returns before the work is done, and the wait shows up under whatever comes next. Wait for it (`llama_synchronize`) before the clock stops.
- **What grows with use:** rows made up for the purpose at a size the person has and at several times it, never the person's own words.
- **Nothing lost:** what the product gives is compared with the old build's, word for word, kept beside the new one.

## The core from a terminal

```
cargo run -p starter-core -- paths | config | update
cargo run -p starter-core -- --try-update <version> <copy of Starter.app>
```
