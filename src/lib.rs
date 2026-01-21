#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/chk/main/logo.png")]

pub use pelican_ui::{Context, event::{OnEvent, Event}, layout::Offset, components::interface::RootInfo};
use pelican_ui::theme::{Color, Theme as PelicanTheme};

pub mod closure;
pub use closure::*;
pub mod flow;
pub use flow::*;
pub mod items;
pub use items::*;
pub mod page;
pub use page::*;

pub enum Theme {
    Dark(Color),
    Light(Color),
    Auto(Color),
}

impl Into<PelicanTheme> for Theme {
    fn into(self) -> PelicanTheme {
        match self {
            Theme::Dark(c) => PelicanTheme::dark(c),
            Theme::Light(c) => PelicanTheme::light(c),
            Theme::Auto(c) => PelicanTheme::from(c)
        }
    }
}

impl Default for Theme { fn default() -> Self {Theme::Dark(Color::from_hex("#ffdd00", 255))} }

pub trait App {
    fn roots(&self, ctx: &mut Context) -> Vec<RootInfo>;
    fn theme(&self, ) -> Theme { Theme::default() }
    fn on_event(&mut self, ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { vec![event] }
}

#[doc(hidden)]
pub mod __private {
    pub use ramp;
    pub use ramp::prism;
    pub use pelican_ui::theme::Theme as PelicanTheme;
    pub use pelican_ui::Context;
    pub use pelican_ui::event::Event;
    pub use pelican_ui::components::interface::Interface;
    pub use chk::App;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
}

#[macro_export]
macro_rules! run {
    ($app:expr) => {
        // TODO: Update state with application support directory files.

        use $crate::__private::*;
        ramp::run!(move |ctx: &mut Context| {
            let app: Rc<RefCell<dyn App>> = Rc::new(RefCell::new(($app)(ctx)));
            *ctx.state.get_mut_or_default::<PelicanTheme>() = app.borrow().theme().into();
            let roots = app.borrow().roots(ctx);
            let app = Rc::clone(&app);
            let on_event = Box::new(move |ctx: &mut Context, event: Box<dyn Event>| app.borrow_mut().on_event(ctx, event));
            Interface::new(ctx, roots, on_event)
        });
    }
}

extern crate self as chk;