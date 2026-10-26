# Design

The standard every Alkaline Software product is held to, in everything a person sees: a window, its pages, sheets, cards, menus and controls, a settings panel, a menu bar item, a site, an icon. The bar is the work that wins Awwwards, the Webbys and the FWA: minimal, calm, intuitive, finished in every state. Every change meets it unasked.

This is the template a new product's `docs/design.md` is written from: the product copies what applies, in its own terms (its accent, its icon's subject, its window's size, the parts it has), leaves out what does not, and from then on its own doc is its standard. A new product takes what applies, in its own terms (its accent, its icon's subject, its window's size, the parts it has), leaves out what does not, and from then on its own doc is its standard.

The products are Rust and GPUI (Zed's toolkit) with the vendored `gpui-component`. The names in brackets are the helpers in `template/crates/starter-app/src/` that draw each pattern (`choice_pill`, `Motion::segmented`, `Motion::presence`), working in the template's window; a new product made from the template has them. A product on another stack draws the same thing by the same rule and names its own helpers.

## Rules

- **Calm.** One accent, for what is on, in use, chosen, or the one thing a page is for. Nothing else competes with it: a page with many of the same action (seven Downloads) draws them quiet. A color that stands for something else (an agent's own color, a model maker's mark) marks that thing and nothing else, and is never an accent.
- **Everything fits.** Every section of a settings page shows all it has without scrolling. The page still scrolls, for a window smaller than the panel, for what a fold opens or what is added later; a new row that would push a section past the panel means the section is reworked, not left to scroll.
- **Nothing jumps.** What appears, goes, opens, closes, grows, moves, fills or turns does so over a moment, with the pattern its neighbors use (Motion). A section's name fades with its page. A jump is a bug.
- **No pause at the end of a motion.** It runs into its place and stops, with no crawl, hesitation or second step in its last frames. Every new animation has had one until it was looked for, so look for it (Motion, No pause at the end).
- **What is cut says so.** Content that runs past an edge fades out at that edge, and can be reached.
- **A gesture stays where it began.** A scroll, its momentum included, moves only the scroller it started in and stops at that scroller's edge; it never carries over into the scroller the pointer reaches next (Scrollers).
- **A motion that repeats runs on the window's clock** (Motion, Frames), never at the display's rate for as long as it runs.
- **Words are few.** A row's name says what it is. A line under it is there only where the name does not say it all, in a few plain words, and never repeats what the control shows. Say what to do once, to someone who can use a computer: no line on how to paste or type a password.
- **Words are for everyone.** A product used in many languages says nothing of how it does in one of them: a model's line says what it is for all, never "weaker Chinese".
- **One way to draw each thing.** A setting is a row on a card; on or off is a switch; a choice among a few is a segmented control; a list of many drops down; a button is a pill with an icon before its word; a face is a tile drawn in itself; a list of many on or off is ticks; a key is a cap. A new control is a new pattern for every page of every product, or it is not made.
- **Things line up.** Names start on one line, controls end on another, and the widest controls on a page start and end together. Every field, button and choice in a row is one height.
- **Every language.** A label never depends on English being short: controls grow to their words, and a page that fits in English fits in French and German. Each language is written as its own Mac writes it, in its own punctuation and with the system's names for its settings.
- **Both appearances, every state.** Light and dark are designed together, never one inverted: every color has a value in each. A change is looked at in both, and in every state it has: empty, full, refused, waiting, off, hover, pressed, mid-motion, a narrow window.
- **Readable.** Words read on their ground at 4.5 to 1 or better, a control's edge at 3 to 1. A key is drawn as a key, never as code. A focus is visible, and every control is reached by the keyboard.
- **Look before saying it is done.** At all the change shows, hides or moves, in both appearances and in a narrow window (Checking).

## The palette

### The neutrals

One set, kept in the app's theme file (`themes/<app>.json`); the site's tokens copy it. Three things are kept apart by more than a shade, because a window of creams a step from each other tired the eye and gave it nowhere to rest: the frame, the page, and the accent.

| Role | Light | Dark |
|---|---|---|
| The page | `#FBFBFA` | `#1A1918` |
| Words (the ink) | `#1F1E1D` | `#F3F1EA` |
| Muted: a line under a name, a date, an icon at rest | `#77746B` | `#A09D94` |
| Panel: a card, a tile, a block of code | `#F0EFED` | `#222120` |
| Border: every hairline | `#DFDFDC` | `#32312E` |
| Frame: the window's ground, the sidebar, a navbar | `#3D3A36` | `#3A3733` |

- **The frame** is a warm graphite in both appearances. Beside a light page it is near black (the ink's own near black was tried and was too heavy); in the dark appearance it is lighter than the page, so the page is a well in a lighter shell and the sidebar is the same thing in both. The sidebar stands on it with no edge of its own and draws with a theme that says it is dark, whatever the page is.
- **The page** is near white with a trace of warmth, its grays (panels, tracks, bubbles, lines) the old creams with most of their color taken out. In the dark appearance the page is the darkest ground there is.
- **A card** is a shade off the page: the muted color at 40% in the light appearance, the sidebar's in the dark. A control on a card takes the muted shade, never the sidebar's, since the dark card is already the sidebar's shade.
- **A track of buttons** (a segmented control, a strip of tabs) is lighter than the page and than a card on it in the dark appearance, or a control has no edge.
- A color that is not in the theme file is a mistake.

### Code's inks

One set of inks an appearance (`look::Inks`) colors everything that colors code or a terminal: the editor, a block of code, a terminal's sixteen named colors. Two sets of colors for one job read as two products.

- The hues are Rosé Pine's (pine, iris, gold, rose, foam, love) with a sage green beside them, each taken darker or lighter until it reads at 4.5 to 1 on the window's own page in either appearance, the gray for comments a step quieter. VS Code's Light+ and Dark+ were the first, and their crimson and pure blue were loud on the page.
- Keywords, tags and a markdown title blue; control keywords, numbers, constants and attributes violet; strings and a code span gold; functions rose; types and properties teal; comments and markdown's marks gray. Variables, operators and brackets stay the page's ink, so a line is not all color.
- Green and red are for a terminal's "added" and "failed"; code does not use them.
- The theme file names an ink where it wants one (`"@l-blue"`, `"@d-rose"`), and the inks are put in before the file is parsed.
- **Inline code** is the accent on a wash of itself: the letters in the link color, on a thin plate of the accent with a border, so both follow the accent the person chose.

### The accent

A product wears one accent at a time, the one color the window has of its own, which is why it stands out. A product with a settings window offers the seven and the person chooses (`look::ACCENTS`); a menu bar app with one window may wear one and have no setting. A product that needs an accent of its own makes one by the same rule.

| Accent | Light: base, hover, active | Dark: base, hover, active |
|---|---|---|
| Periwinkle | `#5061B3` `#4A5BAC` `#4050A0` | `#94A7ED` `#A0B3FB` `#879AE0` |
| Orchid | `#845293` `#7E4C8D` `#724181` | `#C498D2` `#D1A4DF` `#B78BC4` |
| Rose | `#9C4A66` `#954360` `#893856` | `#DA93A9` `#E79FB5` `#CC879C` |
| Amber | `#A86003` `#A05B00` `#905203` | `#DD9B5F` `#EAA76B` `#CF8E52` |
| Moss | `#5A723A` `#556C34` `#4A6129` | `#9CB67C` `#A7C288` `#8FA86F` |
| Jade | `#327860` `#2B725A` `#1E6750` | `#77BCA1` `#83C8AD` `#6AAE95` |
| Teal | `#277580` `#1F6F79` `#0E646E` | `#6FB9C4` `#7BC5D0` `#61ACB7` |

- **One family.** Periwinkle is the icon's navy lifted to read on either page: OKLCH lightness 0.52 and chroma 0.13 in the light appearance, 0.74 and 0.105 in the dark. The other six are other hues at that lightness, so every accent weighs the same on the page. Hover is 0.02 darker in the light appearance and 0.04 lighter in the dark; active is 0.06 and 0.04 darker. A link is the hover shade in the light appearance and the base in the dark.
- **Soft.** At one chroma the greens and the cyan are loud, so their chroma is eased, by most for teal and jade. An orange at 0.52 is brown, so amber's light shade is 0.56, as light as keeps white on it at 4.5 to 1.
- **Even steps, two gaps.** The hues go round the wheel 40 to 66 degrees apart (272, 318, 0, 62, 128, 168, 208) and leave out two places on purpose: the red-orange around hue 40, which is Anthropic's (terracotta `#D97757`, Crail `#C15F3C`), and the blues between teal and periwinkle, which periwinkle stands for.
- **On an accent.** White on the light shade, the ink on the dark one: 4.8 to 5.9 to 1 and 6.9 to 7.5 to 1 across the seven.
- **On the frame.** The light shade is 2 to 1 on graphite, so an accent on the frame (a sidebar's wordmark, a navbar's name) wears its dark shade in both appearances.
- **A new accent** is a hue in a gap that keeps the steps even, at the same lightness, its contrast checked on and off it in both appearances. Graphite was dropped as too pale to mark anything; brass, a gold at 0.52, read as olive.
- **Others' colors.** An agent's own color (Claude's terracotta, Codex's green) and a model maker's mark are shown beside that thing's name and nowhere else. Amber is never text.

A product's own accent is in its `docs/design.md`.

## The type

Faces, each with one job:

- **The chrome** is the system's face: every control, label, tab, chip, cap, date and heading of the window, and what is typed in the window's own fields.
- **The content** of a product whose content is words (a conversation, a document) is in a face of the person's choice, chosen in a Fonts section where each face is a tile drawn in itself. A set that has served: the serifs Source Serif, Newsreader, Literata and Lora, and the sans serifs Inter, IBM Plex Sans, Source Sans and Geist; the first serif to begin with. A page's title and the greeting are in that face too, so a title and what is under it agree.
- **Code** is in one monospace face of the person's choice, the same everywhere the window shows code: a block of code, the editor, the terminal. A set that has served: JetBrains Mono to begin with, then Monaco on a Mac and IBM Plex Mono elsewhere (one choice, whichever face a system has in that place, so a settings file carried between systems keeps its meaning), Maple Mono and Geist Mono. A terminal also carries the Nerd Font symbols (`SymbolsNerdFontMono`), for what a prompt draws.
- **Code among words** is a smaller cut of the monospace face: the same face as a family of its own, made at launch from its bytes with more units to the em, which is what `size-adjust` does in a style sheet. A monospace face is large on its body and stood over the words at their size, and gpui lays a paragraph out at one size, so a smaller code span can only be another face.
- **The name**, in a sidebar's wordmark or an About card, is Optima where the system has it, else Avenir Next (the Mac's own, never carried), else Belleza, a free face with the same flared strokes, carried for Windows and Linux. Optima has only regular and bold, so the regular is drawn twice half a pixel apart for a weight between.
- **Carried faces** are under a license that lets an app carry them (the SIL Open Font License; a math face's GUST license), with that license beside the files in `assets/fonts/`. Google Fonts' static instances: regular, semibold, bold and an italic, and a medium for a mono. A face is a static file a weight: handed a variable font, gpui holds one face of it and draws every bold as regular. Apple's faces (Optima, Monaco) are used from the system and never carried. Anthropic's faces are theirs: not carried, not loaded from the Claude app, not fetched.
- **Sizes in the window**, in points: a section's name 15 semibold; a row's name 13; text in a control 12.5; a group's name 12 semibold in the muted ink; a row's line 11.5 on a line of 15 in the muted ink; a panel's title 13 semibold. Bold is 600, never 700.
- **The content's size** is the person's: a Text Size section with Small, Medium and Large presets for every place at once (the conversation, the composer, a file shown, the editor, the terminal) and Custom, each place its own, in whole points by a stepper and tenths typed. Medium is the Claude desktop app's body size and line height.
- **Sizes on a site**: body 17 px on a line of 1.6 (16 px below 600 wide), in a column of 60 to 75 characters; six sizes of type and nothing in a seventh (the hero's name, h1 2.25 rem, h2 1.5, h3 1.125, body 1.0625, small 0.875 for labels, captions and caps); code at 0.9 em of the words around it.

## Icons in the window

### Tabler for the window

- **Tabler's outline icons** (tabler.io/icons), from the `@tabler/icons` package (`icons/outline/<name>.svg`, and `icons/filled` for the few that are filled), MIT, with `TABLER-LICENSE` beside them. Drawn with a stroke of 1.75 where Tabler's own is 2: at the sizes a window uses, 2 reads heavy. Phosphor's were tried; their corners read as sharp.
- **Each file is under the name the toolkit asks for.** gpui-component ships Lucide's and asks for each by file name (`IconName::Close` is `icons/close.svg`), and the product's asset source answers for a path before the toolkit's own set does. So `assets/icons/` holds a Tabler icon under each toolkit name (`close.svg` is Tabler's `x`, `bot.svg` its `robot-face`), and the icons the toolkit draws by itself change too. Each is written on one line without Tabler's empty bounding path. The folder is built into the app whole (`assets.rs`, by rust-embed), so a file put there is served with no list to keep, and a test fails on an icon named in the code and not carried: one that is not there draws nothing, and nothing says so. No call site names Tabler.
- **Two of Tabler's put together** are still Tabler's: a New File icon is the sheet of one and the plus of another, so New File and New Folder carry the plus in one place.

### Catppuccin for files

- **A file or a folder in a tree** wears Catppuccin's icon for its kind (the VS Code icon theme, `catppuccin-vsc-icons`, MIT, copied from the extension's `dist/`): Latte in the light appearance, Mocha in the dark, with the theme's table cut down to what is used, in `assets/catppuccin/`. Looked up by the whole file name, then the extensions from the longest, a folder by its name, open or closed.
- An icon of this set has its own colors, so it is drawn as a picture (`img`), not as the one-color `Icon`.
- A newer release of the theme is copied over the folder, and the product's own icons and table lines are put back after it.

### Drawing one that neither set has

When neither Tabler nor Catppuccin has the icon a thing needs, it is drawn by hand, in the product's `assets/`:

- **After the thing's own mark, in the hand of the set it stands among**: Tabler's 24-unit box, its line, round ends and 1.75 stroke in the window; Catppuccin's 16-pixel line and fills in a file tree. The template's Quarto icon (`assets/catppuccin/*/quarto.svg`) is Quarto's circle of four quarters drawn as Catppuccin draws, each quarter with a light fill: the line alone read as a wheel, the fill alone stood heavier than its neighbors.
- **Two colors at most, and one is better**, with an accent that fits what it is (a Quarto file takes Markdown's, since it is markdown and more). A mark the window already colors wears that color: an agent's its own.
- **A mark that only looks like the thing is not enough.** Quarto's first, two quarters filled in a ring, read as BMW's.
- **A config file keeps its format's icon** (`_quarto.yml` is YAML's); a name gets a mark of its own only where the file is a kind of its own (`README.md`, `AGENTS.md`).
- It is added to the set's table under its names, and the product's doc says it is its own.

### Others

- **Every button has an icon before its word.** No button has a word and no icon (Controls).
- **A choice in a menu has an icon** too, by what the choice does.
- **The app's own icon** in the window is drawn from the size nearest its use (a 128 for a sidebar, a 512 for About), never one stretched to the other: the 128 at the larger size drew a hairline at the picture's right and foot.
- **Others' marks** (an agent's, a model maker's) are their own, under their own license beside them, and wear their own color beside that thing's name and nowhere else.
- **A flag** is drawn as a picture so it keeps its colors; the flags are drawn by hand in `assets/icons/flags/`.
- **A key is drawn on a cap** (`key_text`): small plates of one height, none narrower than that, each key in its plate's middle, a sign (⌘ ⇧ ⌥ ⌃ 🌐 ↩ ⇥ ↑ ↓) as an icon and a letter as a letter, both of one size. The signs as characters stood at two heights, and as icons on the words' line they were neither the size nor the height of the letter beside them. Three ways in: beside what the key does (`kbd_hint`), in a hover's label (`tip`), and in a line of words (`key_text`). A hover's label names the key that does the same after its words, as the Mac writes it ("Search (⌘K)"); off a Mac ⌘ and ⌥ are the words Ctrl and Alt on their caps. A string with a sign in it is never handed to the window as it is.

## The app icon

Every product's icon is one drawing in one manner, so the Dock reads them as one studio's.

### The manner

- **The ground** is the Alkaline Software cream, a diagonal gradient from `#fdf8ee` at the top left to `#f1e4d0` at the bottom right, computed per pixel: a canvas gradient fill is dithered by the browser, and that noise makes the PNG about ten times larger.
- **The lines** are navy (`#2b2556`), with one amber stroke (`#ffb83e`) and the violet four-point spark (`#6a3fd9`), the mark of the model at work. These three are the studio's and the same in every icon.
- **The product's accent** takes the one place the subject has for a color of its own: periwinkle's dark shade (`#94a7ed`) for the template's folded corner, or a green (`#8cc992`) for a leaf. It stands clear of the navy at 16 pixels (6 to 1 for periwinkle).
- **The subject is one thing**, the product's own, seen plainly: a handscroll from the front with a conversation on its sheet; a leaf that is also a speech bubble, its veins the bars of a sound wave. Lines only, few of them, no words.
- **It is one icon everywhere.** No dark variant: a dark drawing for the dark appearance was made and dropped.

### The drawing

- `scripts/icon/icon.html` is the source: a page that draws the icon on a canvas in a 100-unit box, with proofs of every size beside it, zoomed and at true pixels, on the light and the dark menu bar, and the studio's own icon beside it for the manner. The page is the place to look at it, and the renderer runs the same page.
- **Every size is drawn on its own pixels**, never scaled down from a larger one. The drawing takes the size it is asked for and simplifies as it must: upright strokes land on whole pixels so 32 px stays crisp; below 24 or 32 px across, fine detail (the veins, the lines of a conversation) gives way to the three or four strokes that still read. A treatment that looked the same as the flat fill below 32 px is not worth its weight.
- **Two bodies.** `bleed` fills the whole canvas as a square, opaque to its corners: the `.icns`, since macOS 26 masks every app icon to its own rounded square over a gray backing and shows anything transparent (a margin, rounder corners) as a gray border. Otherwise the body is an inset squircle with a margin and a shadow (`rgba(20, 16, 50, 0.28)`, blur 4.5% of the size, offset down 2.2%): the PNGs the window and the site show.
- **The menu bar's image**, for a menu bar app, comes from the same page: the subject as one black silhouette with one feature cut out of it (the midrib of the leaf), on a transparent square, scaled to fill it. The system tints it, so only its alpha counts.
- **Chosen from candidates.** The page holds a `VARIANT` switch and draws each candidate in the subject's place; the one in use is named in the product's `docs/design.md` with what the others were and why they lost (a symmetric upright leaf read as too stiff; a 45 degree one with slanted veins as too far from the first drawing).

### Rendering

`scripts/make-icon.sh`, on a Mac, runs `scripts/icon/render.swift`, which loads the page in a windowless `WKWebView` and writes what `exportIcon(size, bleed)` or `exportTemplate(size)` returns, once per job, as `<out>.png:<size>:<margin|bleed|template>`:

| Output | Sizes | Body |
|---|---|---|
| `assets/icon/icon-<s>.png`, for the window and the site | 1024, 512, 256, 128, 32 | margin |
| `assets/icon/icon.ico`, Windows, through `scripts/icon/ico.py` | 256, 128, 64, 48, 32, 16 | margin |
| `assets/icon/icon.icns`, macOS, an iconset through `iconutil` | 16, 32, 128, 256, 512, each at 1x and 2x | bleed |
| `assets/icons/menubar.png`, `menubar-1x.png`, a menu bar app's item | 36 (18 pt at 2x), 18 | template |

Nothing else is needed for the icon. A change to it is made on the page, looked at there, rendered again, built, and the assets committed. The other pictures a release carries are drawn by scripts of their own in the studio's cream and the product's accent, and committed: the disk image's background (`scripts/dmg/`), the Windows installer's splash (`scripts/splash/`, the icon and the name over the progress bar Velopack draws), and the Microsoft Store's tiles.

## The window

Laid out like the Claude desktop app, as the Mac's own settings are.

### The frame and the card

- The window's ground is the frame. The sidebar stands on it with no edge of its own, and the content is a card on it with rounded corners: a band of the frame shows over, under and right of the card, and left of it when the sidebar is away.
- The card's corner is the window's less the frame's band, so the two curves run side by side. A rounder window than card thins the frame at each corner. The window's own corner is the system's.
- **A clip in gpui is a rectangle**, `overflow_hidden` too: a ground (a fill, a hover, a chosen row, a header strip, a picture) that reaches a rounded card's corner stands square past its curve. Round it there itself, by the card's radius less its border (17 in a card of 18). What scrolls cannot be rounded, since its edge moves: keep room between the scroller and the corner outside the scroller. Over a solid ground the corner can be painted over instead; a sheet or a menu floats over a page, so it cannot.
- The traffic lights and the buttons beside them are on one line, their centers on the top strip's middle, and do not move whether the sidebar is there or not. The top strip is the app's to drag by, and a tab or a button on it takes its press before the drag region does.
- Everything readable sits in one column, centered in what is left of the window.
- A floating card (a composer, a palette, the settings) lifts with a shadow in the ink's own hue, not a gray halo. Nothing casts a shadow at rest.
- Radii: 8 for a tile, a block of code and a cap; 12 for a card; 18 for a sheet; a full round for a pill.

### The settings

The settings are a window of their own, as the Mac's own settings are, opened by ⌘, the app menu's Settings…, a gear, a menu bar app's item, and any other way in; opened again, the window comes forward on the section asked for. A rail of sections at the left, the section's page at the right, the section's name standing in the window's top level with the traffic lights, both fading in as the section changes. No close button: the window closes as any window does, and with the main window where there is one. Its top and the rail's empty room drag it. Tried and dropped: the settings as a sheet over the main window, which covered what it set.

The window is 820 wide and as tall as its longest section needs as it opens (600 for the template's three sections), never narrower than it opens, since its rows are laid out for that width (narrower, names broke mid-word), and down to 460 tall, where the page scrolls. Every section fits without scrolling (Rules): a section that would not is split, and two that read as one are merged (Updates goes into About).

| Part | Size |
|---|---|
| The rail | 190 wide; its rows start under the traffic lights; a row a section, 30 tall, an icon and a name, the chosen one on a plate |
| The page's margin | 36 at either side |
| The top band | 52 tall; the section's name 15 semibold |
| Groups | 18 apart |
| A group's name | 12 semibold in the muted ink, 6 over its card, in line with the rows' names (14 in from the card's edge) |
| A card | rounded 12, a hairline edge in the border's color, the card's ground (The neutrals); rows parted by the same hairline; 14 of padding in place of rows when it holds tiles or ticks |
| A row | at least 46 tall, 14 in at either side, 8 above and below; its name 13; its line 11.5 on 15 in the muted ink; its control at the right |
| A control | 28 tall, its text 12.5; the widest 336 (`CONTROL_W`): a field of 160 and two pills of 80, or a field and one pill filling the rest |
| A switch | 36 by 20, its knob 16; the track the switch color off and the accent on, both over a moment |

- The rail stands on the muted shade at half in the light appearance and the sidebar's in the dark. A section with something waiting (an update, a permission not given) wears a dot in the accent on its rail row.
- What belongs to a whole group (a lane's switch, whose sessions are shown) stands at the group's name's right, in line with the controls.
- What a row says of itself later (a key not taken, a guide) folds open under it, across the card, in motion.
- A row that paints its own ground (the one in use, washed with the accent) rounds it where it meets the card's corners.
- An accent's swatches are discs of one size, the chosen one marked by a ring inside it in the card's ground, its name the row's line.
- A new row is made with `line` in a `group` (`settings.rs`), never laid out by hand.
- **One state, two windows.** The settings window draws the main view's own state, so every listener is the main view's and a change shows in both windows at once. A list or a menu opened in it is drawn in it, and a field made for it is made with it, since an input watches the window it was made in.
- The last section, About, is one card holding the title card, set as a film's first one is: "Alkaline Software presents", the app's icon, its name in the name's face and the accent, the version in small gray words; under it a sentence naming its owners, each name a link, and pills to the site, the source and the release's notes. Under the credits, the updates: a card of two rows, Updates (what a check found, and the one button that checks or installs) and Check automatically. No gradients.

### Controls

- **A button** is a pill with an icon before its word (`choice_pill`), in one of three kinds (`Press`): quiet, the default, for one of several and for the way out; primary, filled with the accent, for the one thing a page waits on (Allow, Update to x); danger, for what cannot be taken back (Remove, Delete All), which asks once more where much is lost. On a strip or a foot line it is the smaller pill (`small_pill`). Every choice on a card that asks something is one, and so is every new button. The toolkit's `Button` is used only for a button that is an icon alone (a sheet's close); with a label it is never used, since its outline style sets its own ink under the pointer and turned a danger's red black.
- **A switch** is for on or off, at a press anywhere on it. It took the place of two words as a choice, On in green and Off in red.
- **A segmented control** is for a choice among a few: a muted track, the choice on a raised plate that slides from the old choice to the new. The three looks in Appearance have an icon each.
- **A list of many** drops down from a pill; the one list that opens downward only is the one at a page's foot. The toolkit's popover has no way out but at once, so the list is the product's own.
- **A tick** is for a list of many on or off, where a switch a row would be too much.
- **A field** sits in a wrapper that tracks the focus and carries the text role, so keys and assistive apps reach it. A field that grows with what is typed changes the page's length: a long text scrolls inside itself at a height the page leaves it.
- **A hover's label** is the toolkit's managed tooltip, which comes in and slides to the next element on the row, never the plain tooltip that jumps in and out; a row with both read as two kinds of control.
- **A scroller**, every one, has the toolkit's bar and keeps its own gesture (Scrollers).
- **A notice** (something refused, a model to download first) opens down from the section's title and closes after a while.
- **A sheet over the window** (a list to edit, a history, a rename) is smaller than the window both ways, on a scrim any click or Escape puts away, rising in as it fades in and fading out where it was. The sheet occludes, not only its scrim, or a press in the sheet let go outside it closes it.
- **A menu** at a right click is the product's own card at the pointer, kept inside the window, over a clear sheet any click or Escape puts away: each choice with an icon before its words, choices of one kind together with a line between the groups (what opens the thing, what it gives the clipboard, what changes it), the card as wide as its longest choice and no wider, so the room at its right is the room at its left. A text input offers Cut, Copy, Paste and Select All the same way, each with its icon.
- **A key** shown anywhere is on a cap (Icons in the window).

### Scrollers

Every scroller in the window, now and later (a list, a box, a card, a menu, a page), has the same bar and keeps its own gesture.

- **The bar** is the toolkit's `vertical_scrollbar` on the scroller's handle, from the vendored `scrollbar.rs`, and nothing else: no scroller goes without one and none draws its own. It shows while the scroller moves, stays one second after the last movement and fades over half a second. Off a Mac its thumb keeps its length at the ends, and on Windows it is a thin line until the pointer is on it (`docs/platforms.md`).
- **A gesture belongs to the scroller it began in, its momentum included.** macOS goes on sending wheel events after the finger lifts, addressed to wherever the pointer is by then, and gpui hands each to the scroller under it; a momentum event is only `Moved`, so a flick in one pane followed by a move to another scrolled the second. A router runs in the capture phase (`Scrollers::route` in `scroll.rs`, from a raw listener a zero-size `canvas` registers at paint, before any scroller), and the scroller under the pointer at the gesture's `Started` phase, or at the first event after a short gap (a mouse wheel sends no phases), owns it. An event that lands anywhere else is applied to the owner and stopped. At the owner's edge the gesture stops; it never passes to the scroller around it.
- **A scroller names itself to the router** as it is drawn (`Scrollers::name`): a pane beside another, a box inside a page, a list on a sheet or a menu over the window. Of those under the pointer with something to scroll, the one drawn last is on top and takes the gesture. A scroller that does neither leaks: a flick in it scrolls whatever the pointer reaches next.
- **A scroller inside a scroller** (a tool call's body in a conversation) is chained: a stroke that begins at its edge going outward is the outer one's, and one that moved the inner stays with it. The template's router does not chain; a product that nests scrollers adds it there.
- **A sideways scroller** (a strip of tabs) is not named: a sideways flick moves nothing else. What runs past its edge fades into the ground (Motion).
- **A scroll a click starts** glides (Motion), never jumps.
- Rows inside a scrolling flex column go in a column of their own and never shrink: as direct children each gave up height once the list was longer than its room, and the spacing changed at a click.

### What the window remembers

Its bounds, the sidebar and its width, the open tabs and the active one, the open folders, the panels: written when any of that changes (bounds at most every two seconds), at quit, and when the window's close button closes it (on Windows and Linux closing the window ends the app without a quit's save). The bounds are reused only when their center is still on a screen, and are what `window_bounds()` says, never `bounds()` (`docs/platforms.md`, Windows). The page is saved but not restored: a launch opens on the first page. Scroll positions are not saved.

## The menu bar

For a product that lives in the menu bar.

- **A menu bar app** has `LSUIElement` in every bundle and asks the system to stay in the menu bar at launch, because gpui makes every app a regular one as it starts. While its window is open it is a regular app: in the Dock and ⌘-Tab, with its own menus. Otherwise its window, once another app was in front, stood under that app's windows with no way back to it but the menu bar.
- **The item** is at the menu bar's right: the icon's subject as a template image (The app icon), 18 pt, drawn at 1x and 2x, which the system tints for the light bar, the dark one and the highlighted state.
- **A left click opens the settings** where they were left; a right click, or Control and a click, shows the menu. The person can swap the two clicks and can hide the item; both change at once. Opened again from Finder or Spotlight while running, the app shows the settings, which is the way back with the item hidden.
- **The menu is the system's own**, never one drawn by the app: the settings' sections in the rail's order with the rail's icons, a line between the sections and Updates and About, another before Quit. The item has no menu set on it, or the system would show it for either click: the button sends both clicks to a target, which opens the settings or sets the menu, clicks the button and takes the menu off again once it closes.
- **Its icons are the window's Tabler SVGs** rendered at one and two pixels a point into template images, so the system tints them for the light, the dark and the highlighted line. An app built on the macOS 27 SDK has its menus' images hidden unless each item asks for its own (`preferredImageVisibility`); without that the menu shows names alone.
- **The menu is named again** when the window's language changes.
- **Launch at login** is the system's to keep (`SMAppService`): read from the system when the window opens or comes back, never written to the settings. The person may turn it off in Login Items.
- **Permissions** are asked when the person presses Allow, never on a timer, each row saying whether it is given and the one thing to do when it is not. When the system's question or System Settings are done, the settings come back in front, where the next permission is. Whether a permission is given is whether the thing it allows can be done (the tap made), tried after the press and at each change the system reports: `AXIsProcessTrusted` answers a running process with its launch-time value. For Accessibility, Allow first forgets the entry the system keeps for the app (`tccutil reset`): the system binds a grant to the signature, and an entry made for an earlier copy signed otherwise shows as on and counts for nothing.
- **Something the app shows over every other app** (a pill while listening) is a borderless non-activating panel that ignores the pointer, so the app in front keeps the keyboard.

## Motion

Nothing in a window or on a page appears or disappears from one frame to the next. What follows is each pattern in use; a new thing takes the pattern its neighbors use, and a change that needs a new pattern writes it into the product's own doc, and here for the next product.

### Timing

Every motion eases with gpui's `ease_out_quint`; on the web the same curve is `cubic-bezier(0.23, 1, 0.32, 1)`.

| What | ms |
|---|---|
| A menu or a list coming in | 140 |
| A menu or a list going out | 110 |
| A hover's label, a link's color (web) | 160 |
| A fold, a row that comes or goes, something small that is there or is not, a panel's width, a sidebar's folder | 200 |
| A tab, a floating sidebar, a segmented control's plate, a sheet rising in | 220 |
| A page's content fading and rising in (window) | 240 |
| A page's content fading in (web) | 400 |
| A scrollbar: stays after the last movement, then fades | 1000, then 500 |

A fade's curve must fit its length: a curve made for a longer fade held the scrollbar at full strength and then cut it.

### Three rules of every pattern

- **The first sight plays nothing.** A folder drawn for the first time, a fold first seen, the window's first draw: nothing moves. A change is played once, keyed on what changed (a serial for each opening of a menu, the new choice of a segmented control), so a control drawn again later does not play its change over.
- **Going is drawn from what it last showed.** What was closed is kept for the moment of its going (the menu, the card, the row, the line's last words) and drawn once more, taking no click meanwhile.
- **The room moves with the thing.** The gap under a card, the air over a line, the column's gap around a fold are inside what opens and closes, so what is around it slides and does not jump. A column's gap outlives a box shut to nothing.

### The patterns

- **A menu or a list over the window** comes in rising a few pixels into place as it fades in, and goes out fading where it was.
- **A sheet over the window** rises as it fades in and fades out where it was. Something that lies over the window with no way out of its own (a rename, a lightbox, a question) fades in once each time it appears and goes at once.
- **A choice that moves** (a segmented control, a strip's buttons, a row of tabs): one plate slides from the old to the new, placed by sum and never measured, on the same clock the row's widths use so it keeps its tab while they move. Between two tracks a plate fades out of one and into the other: a plate does not leave its track.
- **A switch**: the knob slides as the track fills with the accent.
- **A fold** (what opens under a row, a tool call's body, a thought): the box's height runs up from nothing and back, the content fading with it. The content keeps its own height, cut by the box, and says how tall it is as it is drawn.
- **A card whose height is its content's** measures itself as it is drawn and takes no room until that is known. Held by its foot, it opens up from the end and closes back to it.
- **A panel at a conversation's side**: its width opens and closes. One put away for want of room goes the same way, through the call its button makes, never by its flag alone, which left the button lit and the panel gone between two frames.
- **A sidebar** slides in from the window's edge as its room in the row opens, and out as it closes, whoever asked: its button, the key, a window dragged narrower, a panel that needed its room. It is drawn off the clock, not as one element's animation, because the top strip's room, the panels' left and their edges all stand by its width. Over a scrim in a narrow window it slides the same way and the scrim fades. Not played: a drag of its own edge.
- **A row that comes or goes in a list**: one that was not there grows in; one that is no longer is drawn a moment more where it stood, shrinking. A row that only changes place is not moved in motion.
- **Something small that is there or is not** (a bar beside a row, a line under a group, an arrow at a strip's end): `Motion::presence`, asked at every draw, which plays a change and not the first sight. Something whose room stays runs only its opacity.
- **A line under a conversation** (the agent at work, waiting, a card left behind) opens up from the composer and closes back, the air over it part of what moves, keyed by the session so a change of tab plays nothing.
- **A bar in the flow** (a find bar): its height opens with what is under it moving.
- **A scroll a click starts** (an arrow, "Show", "New messages") runs from where the scroller is, a share of what is left each frame, so it slows onto the place. Never a jump straight to it.
- **What runs past an edge of a sideways scroller** fades into the ground there, and the fade comes and goes with there being more that way. What is cut says so and can be reached.
- **Each page's content fades in** when it arrives, keyed on the page; a section's name fades with its page; a window changing its language fades into the new one.
- **A notice** opens down from a title and closes after a while.
- **A card on a site rises** as it comes into view, once; a figure winds under the pointer and follows on a spring, a little past and back, and goes to rest when the pointer leaves, asking for a frame only while it moves.

### No pause at the end

A motion runs into its place and stops there. A crawl, a hesitation or a second step in the last frames is a bug, and every new animation has had one until it was looked for. What made them:

- **A box whose width went under its padding.** Layout keeps padding and border as a floor, so the last pixels closed at a crawl. Scale the padding with the width.
- **A term clamped at zero** (`.max(px(0.))`) while the others still ran, so the sum changed slope near the end. Every term of what the element takes up (width, padding, the gap with it, a margin) goes to its end in proportion to the one eased clock.
- **A gap or margin outside the box** that outlives it.
- **Neighbors that make room only once the element is gone**: a second move after the first. They start when it does.

**Never an animation of no length.** gpui divides by the length to know how far one is, and the debug build stops on a value that is not finite: `Animation::new(Duration::ZERO)` took the app down. An element that animates only sometimes keeps a real length and does nothing in its closure when nothing moves.

### Frames

The window is one view, and gpui lays out and paints the whole of a view at every frame it draws. Whatever moves costs the window whole, at the rate it asks.

- **The window's clock** (`ambient.rs`). What moves for as long as something works (a mark turning, dots, a spinner) is drawn where the clock stands (`ambient::phase`), and while one was drawn the window asks for the next step after the frame: about 30 a second, 20 while another app is in use. Never `Animation::repeat` for it: that asks for every frame the display has, and a mark of 14 points turning held a window at 60 frames a second, a fifth of a core and as much again of the window server's, for as long as it turned. A transition (a fold, a card coming up) still asks for every frame while it runs, and is over in a moment.
- **Views the window keeps** (`AnyView::cached`): a long list, a sidebar, a document as it reads. A frame that changes nothing in one replays its last layout and paint. What it is drawn from must reach it: gpui draws a kept view afresh only when it or something drawn in it notifies, never when the view around it does. It keeps the opacity it was painted at, so one painted during a fade stays faint unless it is drawn afresh at every frame of the fade.
- **A frame that is not a change** (a cursor's blink) goes through the clock's view, which draws the window and tells the main view nothing.

### Left as they are, on purpose

A page's content changing (the content is the thing, and a cross-fade of two long lists is a blur); text arriving in a live reply; a hover's ground; a row changing place in a list when its content is written to; a drag of a sidebar's edge. On the web, nothing moves for someone who asked the system for less motion.

### Traps

- A hover style that changes the text's color is set whatever state the element is in, never under a `when`: gpui keeps the "over" flag only while a hover style is set, and a segment came back to rest lit.
- A measuring `canvas` laid over an element is `.absolute().inset_0()`; with `.size_full()` alone it stands after the element in the flow.
- A wrapper that clips a track let out with its tab stays until the plate's move is over: taken off sooner, the plate under it is a new element and plays its move again.
- gpui holds an inactive window's animations to 30 frames a second to save energy. A window that animates while never active (a list over every other app, which must not take the keyboard from the app in front) opts out with `inactive_frame_interval: None` on its `WindowOptions`; at 30 a 150 ms slide was a jump with one smeared frame.
- `ease_out_quint` over a short slide is a jump: over 150 ms it does most of the travel in its first 40. A short motion of a plate takes a cubic ease, and words whose color changes with it take the color only as far as the plate covers them.
- A border is inside the box. A scroller given exactly its rows' room overflows by its hairline, and `scroll_to_item` then nudges the whole list by that much at the first and last row: count the edge in the height.
- A transition runs on gpui's own clock (`with_animation`, keyed so it plays once), never on a frame requested at every draw while something moves: a list did that first and the slide was rough. A motion that repeats runs on the window's clock (Frames).
- A sheet on a scrim that closes at a click occludes (`.occlude()` on the sheet, not only on the scrim): gpui counts a click where the press and the release are both in an element's bounds, and a scrim's bounds take its sheet in, so a press in the sheet let go outside it closed the sheet.
- A click the window swallows ends the toolkit's text drag (`swallow_click`): the selection layer begins a drag on every left press and ends it on the release, and a handler that stops the release leaves every later move extending a selection.

## Languages

For a product used in many languages.

- The window speaks English, Chinese (Simplified and Traditional, told apart by script or region), Japanese, Korean, French, German and Spanish, and follows the system by default: the first of its preferred languages the product has, else English.
- Only the window's own words change. What another program says (an agent, git, the system), names, paths and keys stay as they are, and so do the AI era's terms in every language: model, agent, subagent, token, prompt, context, mode, effort, thinking, tool call, MCP, hook, skill, plugin. A label that is the term is the English ("Model"), and a translated sentence holds the English word ("切换 model"), never 模型 or Kontext. Older words (session, file, branch, terminal) are translated.
- A sentence is never built from translated pieces: one sentence is one phrase, its values in named blanks.
- English is the key. A phrase is written in the code in English inside `t("…")`, with blanks for what varies and a plural form where a count is said, and one table holds its other languages. A sentence with links in it is woven from its translation, so each language puts the links where its order wants them. A test fails on a phrase asked for and not in the table, one in the table no longer asked for, a language missing, or a blank that differs from the English.
- On a Mac the choice is the app's own `AppleLanguages`, the setting System Settings writes, so the two are one, and the bundle declares its languages so the system offers them. The system's questions (the microphone's) are asked in the chosen language.
- A language is chosen from a list at the right of the first section's title: System first, then each by its flag and letters.
- French and German run longest: a control holds them by growing to its word, never by a width its English fits. Check a change in French, German and Chinese.
- A choice takes at once: the words are read from the table at every phrase, the menus are made again, and the settings window fades into the new language.

## Checking

Shoot every state a change touches, in both appearances and in a narrow window, and look at it whole and close up (each product's `docs/probing.md` says how to drive its window from a script). Then go round again: what is out of line, louder than it needs to be, or not consistent with its neighbors is fixed and shot again, until nothing is left to change. A page that reads well but breaks a rule here is not done.
