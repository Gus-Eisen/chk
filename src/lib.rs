#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/chk/main/logo.png")]

mod structs;
use structs::*;
mod state;
// pub mod examples;
mod flow;
pub use flow::{PageBuilder};
mod pages;

pub use chk::flow::Flow;

pub use chk::structs::{
    Root,
    RootContent,
    Display,
    ListItem,
    Action,
    TableItem,
    Input,
    EnumItem,
    ChecklistItem,
};

pub use chk::pages::{
    PageType, 
    RootPage,
    Bumper,
    RootBumper,
    AppPage,
    BuildablePage,
};

pub use pelican_ui::{
    Context,
    theme::{Theme as PelicanTheme, Color},
    components::TextInputEvent,
    components::avatar::{AvatarContent, AvatarIconStyle},
    components::interface::{RootInfo, Interface, AppPage as PelicanAppPage},
    components::list_item::ListItemSection,
    utils::Timestamp,
};

pub use ramp::prism::{State, layout::Offset, event::*};

pub enum Theme {
    Dark(Color),
    Light(Color),
    Auto(Color),
}

impl Default for Theme {
    fn default() -> Self {Theme::Dark(Color::from_hex("#ffdd00", 255))}
}

pub trait Application {
    fn roots(ctx: &mut Context) -> Vec<Root>;
    fn theme() -> Theme { Theme::default() }
    fn on_event() -> Box<dyn FnMut(&mut Context, Box<dyn Event>) -> Vec<Box<dyn Event>>> {
        Box::new(|_, e: Box<dyn Event>| vec![e])
    }

    fn build(&self, ctx: &mut Context) -> Interface {
        let roots = Self::roots(ctx);
        ctx.state.insert(match Self::theme() {
            Theme::Dark(c) => PelicanTheme::dark(c),
            Theme::Light(c) => PelicanTheme::light(c),
            Theme::Auto(c) => PelicanTheme::from(c),
        });

        let roots: Vec<RootInfo> = roots.into_iter().map(|mut r| {
            let title = r.page.title.clone();
            match r.content {
                RootContent::Avatar(content) => RootInfo::avatar(content, &title, Box::new(r.page.build(ctx)) as Box<dyn PelicanAppPage>),
                RootContent::Icon(icon) => RootInfo::icon(&icon, &title, Box::new(r.page.build(ctx)) as Box<dyn PelicanAppPage>),
            }
        }).collect();

        Interface::new(ctx, roots, Self::on_event())
    }
}


#[doc(hidden)]
pub mod __private {
    pub use ramp;
    pub use ramp::prism;
}


#[macro_export]
macro_rules! run {
    ($app:expr) => {
        pub use $crate::__private::*;
        ramp::run!(|ctx: &mut Context| {
            let app = $app;
            app.build(ctx)
        });
    };
}

extern crate self as chk;
