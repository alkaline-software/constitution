# Performance

How a product is kept fast and light: what a pass of optimization aims at, how one is done, the rules passes have left, and the log each product keeps so the next pass starts where the last one stopped. The frame rules (the window's clock, kept views, nothing slow while a frame is built) are in `docs/design.md`, Frames; measuring from a script is in `docs/structure.md`, Probing.

## Rules

- **A pass has three aims:** the best speed; the least use of the computer (CPU, memory, power, wakeups, disk, network, processes); and nothing the product does lost. The third is never traded for the other two. A change that saves time and changes what a person sees or can do is not made, and every pass ends by checking that it was not.
- **Measured before and after.** A stopwatch in a copy or a throwaway test, taken out when done, timing the real call the same way both times. The first guess is often wrong: a change that looked right saved nothing until it was timed. The numbers go in the log.
- **Incremental, by a log.** Each product keeps `docs/performance.md` with a log of its passes, each entry naming the commit it covers to. A pass reads the last entry and looks only at what changed since that commit, plus anything an entry left for later. A product with no log, or an empty one, gets a pass over everything. A pass that is not logged is lost to the next one, which then does it again.
- **A row's call runs once a row a frame.** What a list's row asks for while it is drawn is asked, in a list of 300 rows at 120 frames a second, 36,000 times a second. A lookup that costs two microseconds is then most of a frame. Count what a row asks, not only what the view asks once.
- **A lookup made at every frame is answered from memory.** A small file the window reads per row or per frame (a list of saved things, a record of what another machine said) is held in memory. The process's own writes go into the memory as they go to the disk, so nothing it writes is ever shown stale. The disk is looked at no more than every half second, for another process's write, so nothing waits long on that either. A read hands out a shared list (an `Arc`) and copies nothing; a lookup by id copies the one item, never the list.
- **A path is a name, not a look at the disk.** The function every path starts from runs many times a frame. Whatever it once did to the disk (a folder moved from an old name, a directory made) is done once a run, not at every call.
- **A loop off the main thread costs on both ends.** A watch that polls another machine or a folder is timed by what one turn costs there and here, and how often it turns. A clock or a check that runs only while its section shows stops when it is hidden.

## A pass

1. Read the product's log: the last entry's commit, and what it left for later. With no log, the pass covers everything.
2. List what is new since that commit (`git log`, `git diff`): the files, and the parts of the window they draw.
3. Find the hot paths in it. What `render` and each row call: a disk read, a system call, a lock, a copy of a list, a parse. Clocks and timers: does anything wake while nothing moves. The main thread: does it wait. Loops off it: how often they turn, and what each turn costs. What grows with use: memory, a list drawn whole, files kept.
4. Time each suspect as it is now.
5. Fix the cause, in the place every caller goes through, not in each caller.
6. Time it again, the same way.
7. Check that nothing is lost: the tests, and a probe copy of each part changed, in each state it has.
8. Write the log entry.

## The log

Newest first, in the product's `docs/performance.md` under `## Log`. An entry is:

- the version and the date;
- the commits it covers, as `<last entry's commit or tag>..` the commit that adds the entry;
- what was looked at, by file and by part of the window;
- what was found and done, with the numbers before and after in a table;
- what was looked at and left as it is, with why, so the next pass does not look again;
- what was left for later, or "nothing".

A product that had passes before it kept a log starts it with one entry, "Before the log", naming the version those passes covered to; the first logged pass starts there.
