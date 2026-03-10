#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/chk/main/logo.png")]

pub use pelican_ui::{Context, event::{OnEvent, Event}, layout::Offset, interface::navigation::RootInfo, theme::Color, image};
use pelican_ui::theme::Theme as PelicanTheme;

pub mod closure;
pub use closure::*;
pub mod flow;
pub use flow::*;
pub mod items;
pub use items::*;
pub mod page;
pub use page::*;

#[derive(Clone, Debug)]
pub struct ChkBuilder(PelicanTheme);
impl ChkBuilder {
    pub fn new(theme: &PelicanTheme) -> Self {Self(theme.clone())}
    pub fn theme(&self) -> &PelicanTheme {&self.0}
}

#[derive(Clone, Copy, Debug)]
pub enum Theme {
    Dark(Color),
    Light(Color),
    Auto(Color),
}

impl Theme {
    pub fn to_pelican(self, assets: &include_dir::Dir<'static>) -> PelicanTheme {
        match self {
            Theme::Dark(c) => PelicanTheme::dark(assets, c),
            Theme::Light(c) => PelicanTheme::light(assets, c),
            Theme::Auto(c) => PelicanTheme::from(assets, c)
        }
    }
}

impl Default for Theme { fn default() -> Self {Theme::Dark(Color::from_hex("#ffdd00", 255))} }

pub trait App {
    fn roots(&self, ctx: &mut Context, theme: &ChkBuilder) -> Vec<RootInfo>;
    fn theme(&self) -> Theme { Theme::default() }
    fn on_event(&mut self, _ctx: &mut Context, event: Box<dyn Event>) -> Vec<Box<dyn Event>> { vec![event] }
}

#[doc(hidden)]
pub mod __private {
    pub use ramp;
    pub use ramp::prism;
    pub use pelican_ui::theme::Theme as PelicanTheme;
    pub use pelican_ui::Context;
    pub use pelican_ui::event::Event;
    pub use pelican_ui::interface::general::Interface;
    pub use chk::App;
    pub use chk::ChkBuilder;
    pub use chk::Theme;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
}

#[macro_export]
macro_rules! run {
    ($app:expr) => {
        // TODO: Update state with application support directory files.

        use $crate::__private::*;
        ramp::run!(move |ctx: &mut Context, assets: Assets| {
            let app: Rc<RefCell<dyn App>> = Rc::new(RefCell::new(($app)(ctx)));
            let theme: PelicanTheme = app.borrow().theme().to_pelican(assets.all());
            let roots = app.borrow().roots(ctx, &ChkBuilder::new(theme));
            let app = Rc::clone(&app);
            let on_event = Box::new(move |d: &mut Box<dyn Drawable>, ctx: &mut Context, event: Box<dyn Event>| app.borrow_mut().on_event(ctx, event));
            Interface::new(theme, roots, on_event)
        });
    }
}

extern crate self as chk;