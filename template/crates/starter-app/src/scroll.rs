//! A scroll gesture belongs to the scroller it began in, its momentum
//! included, wherever the pointer is carried, and stops at that
//! scroller's edge.
//!
//! macOS goes on sending wheel events after the finger lifts, addressed
//! to wherever the pointer is by then, and gpui hands each to the
//! scroller under it. A momentum event is only `Moved`, so a flick in one
//! list followed by a move onto another scrolled the second. The router
//! runs in the capture phase, before any scroller: the scroller under the
//! pointer when a gesture begins (its `Started` phase, or the first event
//! after a short gap, since a mouse wheel sends no phases) owns it, and
//! every event of the gesture is applied to the owner, held to its ends,
//! and stopped.
//!
//! Every scroller that goes up and down names itself as it is drawn
//! (`name`). One that does not leaks: a flick in it scrolls whatever the
//! pointer reaches next. A sideways scroller is not named.
//!
//! A place that scrolls by a count of its own and not by a handle (a
//! terminal's rows back, a code editor that keeps its own offset) names
//! itself too, by where it is and what moves it (`name_own`), and owns a
//! gesture begun over it the same way: while the pointer stays on it the
//! wheel is its own listener's; carried off it, each event is handed to
//! what moves it (`Route::Give`) and nothing else hears it.
//!
//! A gesture begun over nothing that scrolls (a toolbar, a margin) moves
//! no named scroller it is carried onto. What is not named (a sideways
//! strip) still has the events over it.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{point, px, App, Bounds, Pixels, ScrollHandle, ScrollWheelEvent, TouchPhase, Window};

/// A wheel event this long after the previous one begins a new gesture.
const GAP: Duration = Duration::from_millis(150);

/// What moves a place that scrolls by itself, given a wheel's event.
pub type Wheel = Rc<dyn Fn(&ScrollWheelEvent, &mut Window, &mut App)>;

/// A scroller as the router knows it: a handle it moves itself, or a
/// place that scrolls by its own count, where it was last laid out.
#[derive(Clone)]
enum Scroller {
    Handle(ScrollHandle),
    Own(Rc<Cell<Bounds<Pixels>>>, Wheel),
}

impl Scroller {
    fn contains(&self, at: &gpui::Point<Pixels>) -> bool {
        match self {
            Scroller::Handle(h) => h.bounds().contains(at),
            Scroller::Own(b, _) => b.get().contains(at),
        }
    }

    /// Whether it has anything to scroll: a place of its own always may.
    fn scrolls(&self) -> bool {
        match self {
            Scroller::Handle(h) => h.max_offset().y > px(0.),
            Scroller::Own(..) => true,
        }
    }
}

/// Who a gesture belongs to: nobody yet, nothing that scrolls, or a
/// scroller.
#[derive(Clone, Default)]
enum Owner {
    #[default]
    None,
    Nowhere,
    Some(Scroller),
}

/// What the router did with an event: let it go on to what is under the
/// pointer, stopped it, or gives it to a place that scrolls by itself.
pub enum Route {
    Pass,
    Stop,
    Give(Wheel),
}

#[derive(Default)]
pub struct Scrollers {
    /// The scrollers drawn in the last frame, in the order drawn: the last
    /// is on top.
    live: RefCell<Vec<Scroller>>,
    /// Who the gesture under way belongs to.
    owner: RefCell<Owner>,
    last: Cell<Option<Instant>>,
}

impl Scrollers {
    /// At the start of a view's draw: the scrollers it names are this
    /// draw's.
    pub fn begin(&self) {
        self.live.borrow_mut().clear();
    }

    /// A scroller drawn now.
    pub fn name(&self, handle: &ScrollHandle) {
        self.live.borrow_mut().push(Scroller::Handle(handle.clone()));
    }

    /// A place drawn now that scrolls by a count of its own: where it is
    /// (set as it is laid out) and what moves it.
    #[allow(dead_code)] // for a place that scrolls by itself; the template has none
    pub fn name_own(&self, bounds: Rc<Cell<Bounds<Pixels>>>, wheel: Wheel) {
        self.live.borrow_mut().push(Scroller::Own(bounds, wheel));
    }

    /// One wheel event, in the capture phase: what to do with it.
    pub fn route(&self, e: &ScrollWheelEvent) -> Route {
        let now = Instant::now();
        let fresh = self.last.get().is_none_or(|at| now.duration_since(at) > GAP);
        self.last.set(Some(now));
        let begins = match e.touch_phase {
            TouchPhase::Started => true,
            TouchPhase::Moved => fresh,
            TouchPhase::Ended | TouchPhase::Cancelled => false,
        };
        let list = self.live.borrow().clone();
        if begins {
            // The top scroller under the pointer with something to scroll.
            let under = list.iter().rev().find(|s| s.contains(&e.position) && s.scrolls()).cloned();
            *self.owner.borrow_mut() = under.map_or(Owner::Nowhere, Owner::Some);
        }
        let owner = self.owner.borrow().clone();
        match owner {
            Owner::None => Route::Pass,
            // Begun over nothing that scrolls: it moves no scroller it is
            // carried onto, and what is not named has it.
            Owner::Nowhere => {
                if list.iter().any(|s| s.contains(&e.position)) {
                    Route::Stop
                } else {
                    Route::Pass
                }
            }
            Owner::Some(Scroller::Own(bounds, wheel)) => {
                if bounds.get().contains(&e.position) {
                    Route::Pass
                } else {
                    Route::Give(wheel)
                }
            }
            Owner::Some(Scroller::Handle(owner)) => {
                let delta = e.delta.pixel_delta(px(20.));
                let y = (owner.offset().y + delta.y).clamp(-owner.max_offset().y, px(0.));
                owner.set_offset(point(owner.offset().x, y));
                Route::Stop
            }
        }
    }
}
