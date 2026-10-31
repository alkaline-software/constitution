//! The main window: one view (`Shell`) that draws the frame, the sidebar
//! on it, and the content as a card with rounded corners. It also holds
//! what the settings window draws (`settings.rs`), so a change shows in
//! both windows at once.
//!
//! The page is a sample of every pattern the standard names, each drawn
//! by the shared helpers (`widgets.rs`): the buttons, a choice, a switch,
//! a fold, a working mark on the window's clock, a list in a box that
//! keeps its own scroll gesture, and a right-click menu. A product
//! replaces the page and keeps the frame.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tooltip::ManagedTooltipExt as _;
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName, Sizable as _};

use starter_core::config::Config;
use starter_core::update;

use crate::ambient::{self, Ambient};
use crate::i18n::{count, t, tf};
use crate::menu::{Entry, Menus};
use crate::scroll::Scrollers;
use crate::ui_state::{Rect, UiState};
use crate::widgets::{choice_pill, page_in, small_pill, tip, Motion, Press, FOLD_ANIM, SLIDE};

actions!(starter, [ToggleSidebar, OpenSettings, Escape]);

/// The key context of the main window.
pub const KEY_CONTEXT: &str = "Shell";

/// The window's ground shows this far around the card.
pub const FRAME: Pixels = px(4.);
/// The card's corner: the window's less the frame's band, so the two
/// curves run side by side. Off a Mac the window's corner is small.
pub const CARD_R: Pixels = px(if cfg!(target_os = "macos") { 12. } else { 4. });
/// The card's top strip, where the traffic lights and the strip's buttons
/// stand on one line.
pub const TITLEBAR_H: Pixels = px(48.);
/// The room the traffic lights take at the window's left.
const TRAFFIC_W: Pixels = px(85.);
const STRIP_BTN: Pixels = px(26.);
pub const SIDEBAR_W: Pixels = px(240.);
/// Everything readable sits in one column this wide, centered.
const CONTENT_W: Pixels = px(640.);
/// The least the window is dragged to.
pub const MIN_W: Pixels = px(720.);

/// How long a working mark takes to go round once.
const TURN: Duration = Duration::from_millis(1100);

pub struct Shell {
    pub(crate) cfg: Config,
    pub(crate) motion: Motion,
    scrollers: Scrollers,
    pub(crate) menus: Menus,
    focus: FocusHandle,
    ambient: Entity<Ambient>,
    /// The native window's ground as last set: AppKit is told of a change
    /// only, never at every frame.
    ground: Cell<Option<Hsla>>,
    sidebar_open: bool,
    /// The sidebar's coming or going: which, and when it began.
    side_anim: Option<(bool, Instant)>,
    side_scroll: ScrollHandle,
    page_scroll: ScrollHandle,
    box_scroll: ScrollHandle,
    choice: &'static str,
    fold_open: bool,
    fold_h: Rc<Cell<f32>>,
    working: bool,
    /// The row a right click was on.
    pressed_row: Option<usize>,
    /// When the window's bounds were last written.
    bounds_saved: Option<Instant>,
    pub(crate) settings: Option<AnyWindowHandle>,
    pub(crate) settings_section: &'static str,
    pub(crate) settings_scroll: ScrollHandle,
    pub(crate) lang_serial: u32,
    pub(crate) update: UpdateRow,
}

/// What the Updates row says and does.
#[derive(Default, Clone)]
pub(crate) struct UpdateRow {
    pub checking: bool,
    pub found: Option<String>,
    /// While a download runs: how far, from 0 to 100.
    pub downloading: Option<u32>,
    pub said: Option<String>,
}

impl Shell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let ui = UiState::load();
        // The bounds are written as they change, at most every two seconds.
        cx.observe_window_bounds(window, |this, window, _| {
            if this.bounds_saved.is_none_or(|at| at.elapsed() > Duration::from_secs(2)) {
                this.bounds_saved = Some(Instant::now());
                this.save_ui(window);
            }
        })
        .detach();
        let shell = Self {
            cfg: Config::load(),
            motion: Motion::default(),
            scrollers: Scrollers::default(),
            menus: Menus::default(),
            focus: cx.focus_handle(),
            ambient: cx.new(|_| Ambient::default()),
            ground: Cell::new(None),
            sidebar_open: ui.sidebar_open.unwrap_or(true),
            side_anim: None,
            side_scroll: ScrollHandle::new(),
            page_scroll: ScrollHandle::new(),
            box_scroll: ScrollHandle::new(),
            choice: "week",
            fold_open: false,
            fold_h: Rc::new(Cell::new(0.)),
            working: false,
            pressed_row: None,
            bounds_saved: None,
            settings: None,
            settings_section: crate::settings::section_named(ui.settings_section.as_deref().unwrap_or("appearance")),
            settings_scroll: ScrollHandle::new(),
            lang_serial: 0,
            update: UpdateRow::default(),
        };
        if shell.cfg.app.check_updates {
            shell.check_updates_later(cx);
        }
        shell
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus
    }

    /// What the window remembers, written now.
    pub fn save_ui(&self, window: &Window) {
        let mut ui = UiState::load();
        // `window_bounds()`, never `bounds()` (`ui_state.rs`).
        if let WindowBounds::Windowed(b) = window.window_bounds() {
            ui.window = Some(Rect { x: f32::from(b.origin.x), y: f32::from(b.origin.y), w: f32::from(b.size.width), h: f32::from(b.size.height) });
        }
        ui.sidebar_open = Some(self.sidebar_open);
        ui.settings_section = Some(self.settings_section.to_string());
        ui.save();
    }

    // -- the sidebar ----------------------------------------------------------

    pub fn toggle_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // From as far as the move before had got, so a second press while
        // it moves turns it back without a jump.
        let begun = match self.side_anim {
            Some((_, at)) if at.elapsed() < SLIDE => Instant::now() - (SLIDE - at.elapsed()),
            _ => Instant::now(),
        };
        self.sidebar_open = !self.sidebar_open;
        self.side_anim = Some((self.sidebar_open, begun));
        self.save_ui(window);
        cx.notify();
    }

    /// How much of the sidebar is beside the content, from 0 to 1, on one
    /// eased clock that every term of the row's layout goes by.
    fn side_t(&self) -> f32 {
        match self.side_anim {
            Some((coming, at)) if at.elapsed() < SLIDE => {
                let t = ease_out_quint()(at.elapsed().as_secs_f32() / SLIDE.as_secs_f32());
                if coming { t } else { 1. - t }
            }
            _ => {
                if self.sidebar_open {
                    1.
                } else {
                    0.
                }
            }
        }
    }

    fn sidebar_moving(&self) -> bool {
        self.side_anim.is_some_and(|(_, at)| at.elapsed() < SLIDE)
    }

    /// The frame's theme: the sidebar stands on the frame, which is a dark
    /// graphite in both appearances, so it draws with a dark theme's inks
    /// whatever the page is.
    fn frame_theme(&self, cx: &App) -> gpui_component::Theme {
        let mut theme = cx.theme().clone();
        if !theme.mode.is_dark() {
            theme.foreground = rgb(0xF3F1EA).into();
            theme.muted_foreground = rgb(0xA09D94).into();
            theme.mode = gpui_component::theme::ThemeMode::Dark;
            theme.primary = Rgba::try_from(crate::look::on_frame(&self.cfg.app.accent)).map(Hsla::from).unwrap_or(theme.primary);
        }
        theme
    }

    fn render_sidebar(&self, width: Pixels, cx: &mut Context<Self>) -> AnyElement {
        let theme = self.frame_theme(cx);
        let frame = cx.theme().title_bar;
        self.scrollers.name(&self.side_scroll);
        let rows = (1..=30).map(|n| {
            h_flex()
                .id(("side-row", n as usize))
                .h(px(28.))
                .px(px(10.))
                .gap(px(8.))
                .items_center()
                .rounded(px(7.))
                .text_size(px(13.))
                .text_color(theme.foreground.opacity(0.9))
                .hover(|s| s.bg(theme.foreground.opacity(0.08)))
                .child(Icon::default().path("icons/file-text.svg").with_size(px(15.)).text_color(theme.muted_foreground))
                .child(tf("Item {n}", &[("n", &n)]))
        });
        // The room it takes of the row is `width`; the sidebar keeps its
        // own width inside it and slides in from the window's edge.
        div()
            .w(width)
            .h_full()
            .flex_shrink_0()
            .overflow_hidden()
            .child(
                v_flex()
                    .w(SIDEBAR_W)
                    .h_full()
                    .ml(width - SIDEBAR_W)
                    .pt(TITLEBAR_H + FRAME)
                    .child(div().px(px(16.)).pb(px(10.)).child(crate::widgets::wordmark("Starter", 20., theme.primary, cx)))
                    .child(
                        div()
                            .relative()
                            .flex_1()
                            .min_h_0()
                            .child(v_flex().id("side-list").size_full().px(px(8.)).pb(px(12.)).overflow_y_scroll().track_scroll(&self.side_scroll).children(rows))
                            // What is cut says so: the list fades into the
                            // frame at an edge with more beyond it.
                            .children(edge_fade(&self.side_scroll, frame, true))
                            .children(edge_fade(&self.side_scroll, frame, false))
                            .vertical_scrollbar(&self.side_scroll),
                    )
                    .child(h_flex().h(px(40.)).px(px(16.)).gap(px(8.)).items_center().text_size(px(12.)).text_color(theme.muted_foreground).child(t("Sidebar")).child(div().flex_1()).child(crate::widgets::kbd_hint("⌘B", &theme))),
            )
            .into_any_element()
    }

    // -- the strip --------------------------------------------------------------

    /// The two buttons beside the traffic lights, drawn once over the
    /// window's corner, so neither moves whether the sidebar is there or
    /// not. Their centers are on the traffic lights'.
    fn render_strip(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let over_side = self.side_t() > 0.5;
        let theme = if over_side { self.frame_theme(cx) } else { cx.theme().clone() };
        let button = |id: &'static str, icon: IconName, label: SharedString| {
            let hover = theme.foreground.opacity(0.07);
            div()
                .id(id)
                .size(STRIP_BTN)
                .rounded(px(6.))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(move |s| s.bg(hover))
                .managed_tooltip(move |window, cx| tip(label.clone(), window, cx))
                .child(Icon::new(icon).with_size(px(16.)).text_color(theme.muted_foreground))
        };
        let left = if cfg!(target_os = "macos") { TRAFFIC_W } else { px(10.) };
        // A newer version waiting: a dot on the gear, seen with the settings
        // shut, as About's in the rail is with them open.
        let waiting = self.update.found.is_some();
        let gear_dot = self.motion.presence("gear-dot", waiting, cx);
        h_flex()
            .absolute()
            .top(px(1.) + FRAME / 2.)
            .left(left)
            .h(TITLEBAR_H)
            .items_center()
            .child(button("strip-sidebar", IconName::PanelLeft, t("Sidebar (⌘B)").into()).on_click(cx.listener(|this, _, window, cx| {
                crate::widgets::swallow_click(window, cx);
                this.toggle_sidebar(window, cx)
            })))
            .child(button("strip-settings", IconName::Settings, t("Settings (⌘,)").into()).relative().children(crate::widgets::waiting_dot(waiting, gear_dot, theme.primary)).on_click(cx.listener(|this, _, window, cx| {
                crate::widgets::swallow_click(window, cx);
                this.open_settings(None, window, cx)
            })))
    }

    /// The window's top is the app's to drag by, and a button on it takes
    /// its press first (`on_click` above stops it).
    fn drag_region(el: Div) -> Div {
        el.on_mouse_down(MouseButton::Left, |ev, window, _| {
            if ev.click_count == 2 {
                window.titlebar_double_click();
            }
        })
        .on_mouse_move(|ev, window, _| {
            if ev.pressed_button == Some(MouseButton::Left) {
                window.start_window_move();
            }
        })
    }

    // -- the page ---------------------------------------------------------------

    fn section(title: SharedString, body: impl IntoElement, theme: &gpui_component::Theme) -> Div {
        v_flex().gap(px(10.)).child(div().text_size(px(12.)).font_weight(FontWeight::SEMIBOLD).text_color(theme.muted_foreground).child(title)).child(body)
    }

    /// A fold: the box's height runs up from nothing and back, the content
    /// fading with it. The content keeps its own height, cut by the box,
    /// and says how tall it is as it is drawn. The first sight plays
    /// nothing.
    fn render_fold(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let theme = cx.theme().clone();
        let moving = self.motion.presence("fold", self.fold_open, cx);
        if !self.fold_open && moving.is_none() {
            return None;
        }
        let measured = self.fold_h.clone();
        let entity = cx.entity().downgrade();
        let content = v_flex()
            .relative()
            .flex_shrink_0()
            // The room between the fold and its head is inside the fold,
            // so it goes with the box.
            .pt(px(10.))
            .gap(px(6.))
            .text_size(px(13.))
            .text_color(theme.muted_foreground)
            .child(t("A fold opens under its row and closes back to it."))
            .child(t("What is around it slides, and nothing jumps."))
            .child(
                canvas(
                    move |b, _, cx| {
                        let h = f32::from(b.size.height);
                        if (measured.get() - h).abs() > 0.5 {
                            measured.set(h);
                            let _ = entity.update(cx, |_, cx| cx.notify());
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .inset_0(),
            );
        let tall = self.fold_h.clone();
        let wrap = v_flex().overflow_hidden().child(content);
        Some(match moving {
            Some((opening, n)) => wrap
                .with_animation(ElementId::NamedInteger("fold".into(), n as u64), Animation::new(FOLD_ANIM).with_easing(ease_out_quint()), move |d, t| {
                    let t = if opening { t } else { 1. - t };
                    d.h(px(tall.get() * t)).opacity(t)
                })
                .into_any_element(),
            None => wrap.into_any_element(),
        })
    }

    /// A mark at work: it turns on the window's clock, so the window is
    /// drawn about 30 times a second while it does and not at all after.
    fn render_working(&self, cx: &App) -> AnyElement {
        let theme = cx.theme();
        if !self.working {
            return div().size(px(16.)).into_any_element();
        }
        let turn = ambient::phase(TURN);
        let breath = ambient::pulse(0.35, 1., ambient::phase(TURN * 2));
        h_flex()
            .gap(px(8.))
            .items_center()
            .child(Icon::default().path("icons/loader-circle.svg").with_size(px(16.)).text_color(theme.primary).rotate(percentage(turn)))
            .child(div().size(px(7.)).rounded_full().bg(theme.primary).opacity(breath))
            .into_any_element()
    }

    fn render_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.theme().clone();
        let buttons = h_flex()
            .gap(px(8.))
            .flex_wrap()
            .child(div().id("tip-copy").managed_tooltip(|window, cx| tip(t("Copy (⌘C)"), window, cx)).child(choice_pill("copy", "icons/copy.svg", t("Copy"), Press::Quiet, false, &theme)))
            .child(choice_pill("save", "icons/check.svg", t("Save"), Press::Primary, false, &theme))
            .child(choice_pill("delete", "icons/trash.svg", t("Delete"), Press::Danger, false, &theme))
            .child(choice_pill("later", "icons/history.svg", t("Not now"), Press::Quiet, true, &theme));
        let choose = self.motion.segmented(
            "range",
            vec![("day", "icons/sun.svg", t("Day").into()), ("week", "icons/calendar.svg", t("Week").into()), ("month", "icons/layout-dashboard.svg", t("Month").into())],
            self.choice,
            false,
            Rc::new(|this: &mut Self, key, _, cx| {
                this.choice = key;
                cx.notify();
            }),
            cx,
        );
        let switch = self.motion.switch(
            "sample",
            self.cfg.app.sample_switch,
            Rc::new(|this: &mut Self, on, _, cx| {
                this.cfg.app.sample_switch = on;
                let _ = Config::edit(|c| c.app.sample_switch = on);
                cx.notify();
            }),
            cx,
        );
        let fold_head = h_flex()
            .id("fold-head")
            .gap(px(6.))
            .items_center()
            .cursor_pointer()
            .text_size(px(13.))
            .on_click(cx.listener(|this, _, _, cx| {
                this.fold_open = !this.fold_open;
                cx.notify();
            }))
            .child(Icon::default().path(if self.fold_open { "icons/chevron-down.svg" } else { "icons/chevron-right.svg" }).with_size(px(14.)).text_color(theme.muted_foreground))
            .child(t("Details"));
        let working = h_flex()
            .gap(px(10.))
            .items_center()
            .child(small_pill("work", if self.working { "icons/pause.svg" } else { "icons/play.svg" }, if self.working { t("Stop") } else { t("Start") }, Press::Quiet, false, &theme).on_click(cx.listener(|this, _, _, cx| {
                this.working = !this.working;
                cx.notify();
            })))
            .child(self.render_working(cx));
        self.scrollers.name(&self.box_scroll);
        let rows = (0..40).map(|i| {
            let lit = self.pressed_row == Some(i);
            h_flex()
                .id(("box-row", i as usize))
                .h(px(30.))
                .px(px(12.))
                .items_center()
                .text_size(px(13.))
                .when(i > 0, |d| d.border_t_1().border_color(theme.border))
                .when(lit, |d| d.bg(theme.foreground.opacity(0.05)))
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(move |this, ev: &MouseDownEvent, window, cx| {
                        crate::widgets::swallow_click(window, cx);
                        this.pressed_row = Some(i);
                        let reveal = crate::sys::REVEAL_LABEL;
                        this.menus.show(
                            "row",
                            ev.position,
                            vec![
                                Entry::choice("open", "icons/external-link.svg", t("Open")),
                                Entry::choice("reveal", "icons/folder.svg", t(reveal)),
                                Entry::Rule,
                                Entry::choice("copy", "icons/copy.svg", t("Copy Name")),
                                Entry::Rule,
                                Entry::choice("rename", "icons/pencil-simple.svg", t("Rename…")),
                                Entry::choice("delete", "icons/trash.svg", t("Delete")),
                            ],
                        );
                        cx.notify();
                    }),
                )
                .child(tf("Row {n}", &[("n", &(i + 1))]))
        });
        let scroll_box = div()
            .relative()
            .h(px(180.))
            .rounded(px(12.))
            .border_1()
            .border_color(theme.border)
            .overflow_hidden()
            .child(v_flex().id("box").size_full().overflow_y_scroll().track_scroll(&self.box_scroll).children(rows))
            .vertical_scrollbar(&self.box_scroll);
        let _ = window;
        v_flex()
            .w(CONTENT_W)
            .max_w_full()
            .mx_auto()
            .py(px(28.))
            .px(px(24.))
            .gap(px(28.))
            .child(div().text_size(px(24.)).font_weight(FontWeight::SEMIBOLD).child(t("The standard, drawn")))
            .child(Self::section(t("Buttons").into(), buttons, &theme))
            .child(Self::section(t("A choice among a few").into(), h_flex().child(choose), &theme))
            .child(Self::section(t("On or off").into(), h_flex().gap(px(12.)).items_center().child(switch).child(div().text_size(px(13.)).child(t("A switch"))), &theme))
            .child(Self::section(t("A fold").into(), v_flex().child(fold_head).children(self.render_fold(cx)), &theme))
            .child(Self::section(t("At work").into(), working, &theme))
            .child(Self::section(count(40, "{n} row in a box", "{n} rows in a box").into(), scroll_box, &theme))
            .into_any_element()
    }

    // -- the settings and updates ------------------------------------------------

    pub fn open_settings(&mut self, section: Option<&'static str>, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(s) = section {
            self.settings_section = s;
        }
        // After this update: opening reads the view this call is inside.
        let weak = cx.entity().downgrade();
        cx.defer(move |cx| crate::settings::open(weak, cx));
        cx.notify();
    }

    fn check_updates_later(&self, cx: &mut Context<Self>) {
        if update::from_store() {
            return;
        }
        let state = update::UpdateState::load();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.);
        if now - state.last_check < 86_400. {
            return;
        }
        cx.spawn(async move |this, cx| {
            let _ = this.update(cx, |this, cx| this.check_updates(cx));
        })
        .detach();
    }

    /// Ask for the newest version, off the main thread.
    pub(crate) fn check_updates(&mut self, cx: &mut Context<Self>) {
        self.update.checking = true;
        self.update.said = None;
        cx.notify();
        let task = cx.background_executor().spawn(async { update::latest() });
        cx.spawn(async move |this, cx| {
            let found = task.await;
            let _ = this.update(cx, |this, cx| {
                this.update.checking = false;
                match found {
                    Ok(latest) => {
                        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.);
                        let _ = update::UpdateState { last_check: now, latest: latest.clone() }.save();
                        if update::is_newer(&latest, update::current_version()) {
                            this.update.found = Some(latest);
                        } else {
                            this.update.found = None;
                            this.update.said = Some(t("Up to date.").into());
                        }
                    }
                    Err(e) => this.update.said = Some(tf("The check did not go through: {why}", &[("why", &e)])),
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Fetch the newest version and put it in place, then quit: the new
    /// app opens once this one has gone.
    pub(crate) fn install_update(&mut self, cx: &mut Context<Self>) {
        let Some(version) = self.update.found.clone() else { return };
        self.update.downloading = Some(0);
        cx.notify();
        let (tx, rx) = std::sync::mpsc::channel::<u32>();
        let task = cx.background_executor().spawn(async move {
            update::download(&version, &|done, total| {
                if let Some(total) = total.filter(|t| *t > 0) {
                    let _ = tx.send((done * 100 / total) as u32);
                }
            })
            .and_then(|file| update::install(&file))
        });
        cx.spawn(async move |this, cx| {
            let ticker = async {
                loop {
                    cx.background_executor().timer(Duration::from_millis(200)).await;
                    let p = rx.try_iter().last();
                    if this.update(cx, |this, cx| {
                        if let Some(p) = p {
                            this.update.downloading = Some(p);
                            cx.notify();
                        }
                    }).is_err() {
                        break;
                    }
                }
            };
            let done = futures_lite_select(task, ticker).await;
            let _ = this.update(cx, |this, cx| {
                this.update.downloading = None;
                match done {
                    Ok(()) => cx.quit(),
                    Err(e) => this.update.said = Some(tf("The update did not go through: {e}", &[("e", &e)])),
                }
                cx.notify();
            });
        })
        .detach();
    }
}

/// A fade at a scroller's top or foot, there while there is more that way.
/// It comes and goes with the scroll, which draws the view at each step.
fn edge_fade(handle: &ScrollHandle, ground: Hsla, top: bool) -> Option<AnyElement> {
    let (at, most) = (-handle.offset().y, handle.max_offset().y);
    let more = if top { at > px(1.) } else { at < most - px(1.) };
    more.then(|| {
        let (from, to) = if top { (ground, ground.opacity(0.)) } else { (ground.opacity(0.), ground) };
        div()
            .absolute()
            .left_0()
            .right_0()
            .h(px(28.))
            .map(|d| if top { d.top_0() } else { d.bottom_0() })
            .bg(linear_gradient(180., linear_color_stop(from, 0.), linear_color_stop(to, 1.)))
            .into_any_element()
    })
}

/// The first of a task and a loop that never ends: the task's answer.
async fn futures_lite_select<T: 'static>(task: Task<T>, ticker: impl std::future::Future<Output = ()>) -> T {
    use std::future::Future as _;
    use std::pin::pin;
    let mut task = pin!(task);
    let mut ticker = pin!(ticker);
    std::future::poll_fn(move |cx| {
        if let std::task::Poll::Ready(v) = task.as_mut().poll(cx) {
            return std::task::Poll::Ready(v);
        }
        let _ = ticker.as_mut().poll(cx);
        std::task::Poll::Pending
    })
    .await
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.scrollers.begin();
        let theme = cx.theme().clone();
        let frame = theme.title_bar;
        if self.ground.get() != Some(frame) {
            self.ground.set(Some(frame));
            crate::sys::set_window_ground(window, frame.to_rgb());
        }
        if self.sidebar_moving() {
            window.request_animation_frame();
        }
        let t_side = self.side_t();
        let side_w = SIDEBAR_W * t_side;
        // The strip leaves room for the traffic lights and its buttons
        // while the sidebar is away, in proportion to the same clock.
        let strip_room = (if cfg!(target_os = "macos") { TRAFFIC_W } else { px(10.) } + STRIP_BTN * 2. + px(8.)) * (1. - t_side);
        self.scrollers.name(&self.page_scroll);
        let page = self.render_page(window, cx);
        let card = v_flex()
            .flex_1()
            .min_w_0()
            .h_full()
            .rounded(CARD_R)
            .bg(theme.background)
            .text_color(theme.foreground)
            .overflow_hidden()
            .child(Self::drag_region(h_flex().h(TITLEBAR_H - FRAME).flex_shrink_0().pl(strip_room).pr(px(12.)).items_center()).child(div().flex_1()))
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .child(div().id("page").size_full().overflow_y_scroll().track_scroll(&self.page_scroll).child(page_in("page", div().child(page))))
                    .children(edge_fade(&self.page_scroll, theme.background, true))
                    .children(edge_fade(&self.page_scroll, theme.background, false))
                    .vertical_scrollbar(&self.page_scroll),
            );
        let pick = Rc::new(|this: &mut Self, what: &'static str, id: SharedString, window: &mut Window, cx: &mut Context<Self>| this.menu_pick(what, id, window, cx));
        let close = Rc::new(|this: &mut Self, cx: &mut Context<Self>| {
            this.pressed_row = None;
            this.menus.close(cx);
            cx.notify();
        });
        let menu = if self.menus.open.as_ref().is_some_and(|m| m.what == "row") || self.menus.gone.as_ref().is_some_and(|(m, _)| m.what == "row") { self.menus.render(pick, close, cx) } else { None };
        let this = cx.entity().downgrade();
        let ambient = self.ambient.clone();
        div()
            .id("shell")
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus)
            .size_full()
            .relative()
            .bg(frame)
            .on_action(cx.listener(|this, _: &ToggleSidebar, window, cx| this.toggle_sidebar(window, cx)))
            .on_action(cx.listener(|this, _: &Escape, _, cx| {
                this.pressed_row = None;
                this.menus.close(cx);
                cx.notify();
            }))
            // The scroll router, a raw listener in the capture phase,
            // registered at paint, before any scroller hears the wheel; and
            // the clock's step, asked for at the end of the frame.
            .child(
                canvas(
                    |_, _, _| (),
                    move |_, _, window, cx| {
                        let router = this.clone();
                        window.on_mouse_event(move |e: &ScrollWheelEvent, phase, _, cx| {
                            if phase == DispatchPhase::Capture {
                                let taken = router.update(cx, |s, _| s.scrollers.route(e)).unwrap_or(false);
                                if taken {
                                    cx.stop_propagation();
                                    let _ = router.update(cx, |_, cx| cx.notify());
                                }
                            }
                        });
                        ambient::after_frame(&ambient, window, cx);
                    },
                )
                .absolute()
                .size_0(),
            )
            .child(h_flex().size_full().items_stretch().child(self.render_sidebar(side_w, cx)).child(div().flex_1().min_w_0().h_full().p(FRAME).pl(if side_w > FRAME { px(0.) } else { FRAME - side_w }).child(card)))
            .child(self.render_strip(cx))
            .children(crate::csd::chrome(window, px(1.) + FRAME / 2. + TITLEBAR_H / 2. - px(12.), |window, _| window.remove_window(), cx))
            .children(menu)
            .child(self.ambient.clone())
    }
}

impl Shell {
    fn menu_pick(&mut self, what: &'static str, id: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if what == "row" && id.as_ref() == "reveal" {
            crate::sys::reveal(&starter_core::paths::config_file());
        }
        let _ = window;
        self.pressed_row = None;
        self.menus.close(cx);
        cx.notify();
    }

    /// For a script: a picture of the window once it is drawn, then quit.
    /// The settings window when it is open, else the main one.
    pub fn shoot_later(&self, path: std::path::PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(Duration::from_millis(900)).await;
            let settings = this.update(cx, |s, _| s.settings).ok().flatten();
            let _ = cx.update(|window, cx| {
                let shot = match settings {
                    Some(h) => h.update(cx, |_, w, _| crate::sys::shoot_window(w, &path)).unwrap_or_else(|e| Err(e.to_string())),
                    None => crate::sys::shoot_window(window, &path),
                };
                if let Err(e) = shot {
                    eprintln!("starter: STARTER_SHOT: {e}");
                }
                cx.quit();
            });
        })
        .detach();
    }

    /// For a script: a fold opened, the sidebar folded, or the settings
    /// shown, by name (`STARTER_GO=fold;sidebar;settings`).
    pub fn go(&mut self, steps: &str, window: &mut Window, cx: &mut Context<Self>) {
        for step in steps.split(';') {
            match step {
                "fold" => self.fold_open = true,
                "sidebar" => self.toggle_sidebar(window, cx),
                "work" => self.working = true,
                // A newer version waits, as a check finding one says: the
                // gear's and About's dots. Nothing is fetched.
                "updatefound" => self.update.found = Some("9.9.9".into()),
                "menu" => {
                    self.pressed_row = Some(2);
                    self.menus.show("row", point(px(420.), px(560.)), vec![Entry::choice("open", "icons/external-link.svg", t("Open")), Entry::Rule, Entry::choice("copy", "icons/copy.svg", t("Copy Name")), Entry::choice("delete", "icons/trash.svg", t("Delete"))]);
                }
                _ => {}
            }
        }
        cx.notify();
    }
}

impl Focusable for Shell {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
