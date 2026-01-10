use ramp::prism;
use pelican_ui::{Context};
use pelican_ui::event::{Event, OnEvent, TickEvent};
use pelican_ui::canvas::Align;
use pelican_ui::layout::{Offset, Stack};
use pelican_ui::drawable::{Drawable, Component, SizedTree};
use pelican_ui::components::avatar::{AvatarContent, AvatarIconStyle};
use pelican_ui::components::interface::AppPage as PelicanAppPage;
use pelican_ui::components::interface::{Header, Bumper as PelicanBumper, Content, Page as PelicanPage};
use pelican_ui::utils::{Callback, ValidationFn};
use pelican_ui::components::text::{TextStyle, TextSize};

use crate::{Action, Input, Display, FnMutClone, NavFn, ValidityFn, EditedFn};
use crate::flow::Flow;
#[derive(Clone)]
pub enum PageType {
    Display {title: String, items: Vec<Display>, branch: Option<(String, Flow)>, bumper: Bumper, offset: Offset, flow_length: usize, next: Option<NavFn>},
    Input {title: String, items: Input, bumper: Bumper, flow_length: usize, next: Option<NavFn>},
    Settings {title: String, items: Vec<Input>, bumper: Bumper, flow_length: usize, next: Option<NavFn>},
}

impl std::fmt::Debug for PageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PageType")
    }
}

impl PageType {
    pub fn success(title: &str, icon: &str, text: &str) -> Self {
        PageType::Display { 
            title: title.to_string(),
            items: vec![Display::icon(icon), Display::Text {text: text.to_string(), size: TextSize::H4, style: TextStyle::Heading, align: Align::Center}], 
            branch: None,
            bumper: Bumper::Done,
            offset: Offset::Center,
            flow_length: 1,
            next: None,
        }
    }

    pub fn review(title: &str, items: Vec<Display>) -> Self {
        PageType::Display { title: title.to_string(), items, branch: None, bumper: Bumper::default(None), offset: Offset::Start, flow_length: 1, next: None}
    }

    pub fn input(title: &str, items: Input, bumper: Bumper) -> Self {
        PageType::Input { title: title.to_string(), items, bumper,flow_length: 1, next: None }
    }

    pub fn display(title: &str, items: Vec<Display>, branch: Option<(String, Flow)>, bumper: Bumper, offset: Offset) -> Self {
        PageType::Display { title: title.to_string(), items, branch, bumper, offset, flow_length: 1, next: None}
    }

    pub fn settings(title: &str, avatar: AvatarContent, text_fields: Vec<(String, String, Box<dyn EditedFn>)>, bumper: Bumper) -> Self {
        let mut items = vec![Input::avatar(avatar, Some(("edit".to_string(), AvatarIconStyle::Secondary)), Some(Action::SelectImage))];
        text_fields.into_iter().for_each(|(i, t, c)| items.push(Input::text(&i, true, None, c)));
        
        PageType::Settings { 
            title: title.to_string(), 
            items,
            bumper,
            flow_length: 1,
            next: None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            PageType::Settings {title, ..} |
            PageType::Display {title, ..} |
            PageType::Input {title, ..} => title.to_string(),
        }
    }

    pub fn flow_length(&mut self) -> &mut usize {
        match self {
            PageType::Settings {flow_length, ..} |
            PageType::Display {flow_length, ..} |
            PageType::Input {flow_length, ..} => flow_length
        }
    }

    pub fn get_nav(&mut self) -> &mut Option<NavFn> {
        match self {
            PageType::Settings {next, ..} |
            PageType::Display {next, ..} |
            PageType::Input {next, ..} => next
        }
    }
}

impl BuildablePage for PageType {
    fn build(&mut self, ctx: &mut Context) -> AppPage {
        let flow_len = *self.flow_length();
        let next = self.get_nav().clone();

        let (offset, content, header_icon) = match self {
            PageType::Display {items, offset, branch, ..} => (*offset, items.iter_mut().filter_map(|di| di.build(ctx)).flatten().collect::<Vec<Box<dyn Drawable>>>(), branch.take()),
            PageType::Input {items, ..} => (Offset::Start, items.build(ctx).unwrap_or_default(), None),
            PageType::Settings {items, ..} => {
                (Offset::Start, items.iter_mut().filter_map(|di| di.build(ctx)).flatten().collect::<Vec<Box<dyn Drawable>>>(), None)
            }
        };


        let bumper = match self {
            PageType::Display {bumper, ..} => bumper,
            PageType::Input {bumper, ..} => bumper,
            PageType::Settings {bumper, ..} => bumper,
        };



        let icon = header_icon.map(|(i, mut f)| (i.to_string(), f.build()));

        let (header, bumper) = match bumper {
            Bumper::Custom {label, action, secondary, is_valid} => {
                let on_click = action.clone();
                let secondary = secondary.clone().map(|(l, a)| (l, Box::new(move |ctx: &mut Context| (a.clone().get())(ctx)) as Callback));
                let action = Box::new(move |ctx: &mut Context| (on_click.clone().get())(ctx));
                let validity_fn = is_valid.clone().map(|mut vfn| Box::new(move |ctx: &mut Context| (vfn)(ctx)) as Box<dyn ValidationFn>);
                let bumper = PelicanBumper::stack(ctx, Some(label), action, secondary, validity_fn);
                let header = Header::stack(ctx, &self.name(), icon);
                (header, Some(bumper))
            },
            Bumper::Default {is_valid} => match next {
                Some(n) => {
                    let validity_fn = is_valid.clone().map(|mut vfn| Box::new(move |ctx: &mut Context| (vfn)(ctx)) as Box<dyn ValidationFn>);
                    let next = n.clone();
                    let bumper = PelicanBumper::stack(ctx, None, Box::new(move |ctx: &mut Context| (next.borrow_mut())(ctx)), None, validity_fn);
                    let header = Header::stack(ctx, &self.name(), icon);
                    (header, Some(bumper))
                }
                None => (Header::stack_end(ctx, &self.name()), Some(PelicanBumper::stack_end(ctx, Some(flow_len))))
            },
            Bumper::Done => (Header::stack_end(ctx, &self.name()), Some(PelicanBumper::stack_end(ctx, Some(flow_len)))),
            Bumper::None => (Header::stack(ctx, &self.name(), icon), None)
        };

        AppPage::new(header, Content::new(offset, content), bumper, self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RootPage {
    pub title: String,
    pub content: Vec<Display>,
    pub header_icon: Option<(String, Box<dyn FnMutClone>)>,
    pub bumper: (RootBumper, Option<RootBumper>),
}

impl RootPage {
    pub fn new(title: &str, content: Vec<Display>, header_icon: Option<(String, Box<dyn FnMutClone>)>, bumper_a: RootBumper, bumper_b: Option<RootBumper>) -> Self {
        RootPage {
            title: title.to_string(),
            content,
            header_icon,
            bumper: (bumper_a, bumper_b)
        }
    }
}

pub trait BuildablePage: std::fmt::Debug {
    fn build(&mut self, ctx: &mut Context) -> AppPage;
}

impl BuildablePage for RootPage {
    fn build(&mut self, ctx: &mut Context) -> AppPage {
        let header_icon = self.header_icon.as_ref().map(|(s, c)| {
            let closure = c.clone_box();
            (s.to_string(), Box::new(move |ctx: &mut Context| (closure.clone_box())(ctx)) as Callback) 
        });
        let header = Header::home(ctx, &self.title, header_icon);
        let content = self.content.clone().iter_mut().filter_map(|di| di.build(ctx)).flatten().collect::<Vec<Box<dyn Drawable>>>();
        let second = self.bumper.1.as_mut().map(|i| i.get());
        let first = self.bumper.0.get();
        let bumper = PelicanBumper::home(ctx, first, second, None);

        let offset = match self.content.first() {
            Some(Display::List {items, ..}) if items.is_empty() => Offset::Center,
            Some(Display::List {..}) => Offset::Start,
            _ if content.len() <= 1 => Offset::Center,
            _ => Offset::Start,
        };

        AppPage::new(header, Content::new(offset, content), Some(bumper), self.clone())
    }
}

#[derive(Component, Debug)]
pub struct AppPage(Stack, pub PelicanPage, #[skip] Box<dyn BuildablePage>);
impl OnEvent for AppPage {
    fn on_event(&mut self, _ctx: &mut Context, _sized: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        // if event.downcast_ref::<TickEvent>().is_some() {
        //     let new = self.2.build(ctx);
        //     if self.1 != new {
        //         self.1 = new;
        //     }
        // }
        // if event.downcast_ref::<StateChangedEvent>().is_some() {
        //     *self = self.2.build(ctx);
        // }

        vec![event]
    }
}

impl PelicanAppPage for AppPage {}

impl AppPage {
    pub fn new(header: Header, content: Content, bumper: Option<PelicanBumper>, page: impl BuildablePage + 'static) -> Self {
        AppPage(Stack::default(), PelicanPage::new(header, content, bumper), Box::new(page))
    }
}

/// Represents a bumper item on the first page of a tab.
#[derive(Debug, Clone)]
pub struct RootBumper(String, Flow);

impl RootBumper {
    pub fn new(label: &str, flow: Flow) -> Self {
        RootBumper(label.to_string(), flow)
    }

    pub fn get(&mut self) -> (String, Callback) {
        (self.0.to_string(), self.1.build())
    }
}

#[derive(Debug, Clone)]
pub enum Bumper {
    Default { is_valid: Option<Box<dyn ValidityFn>> },
    Custom { label: String, action: Action, secondary: Option<(String, Action)>, is_valid: Option<Box<dyn ValidityFn>>},
    Done,
    None,
}

impl Bumper {
    pub fn custom(label: &str, action: Action, is_valid: Option<Box<dyn ValidityFn>>) -> Self {
        Bumper::Custom {label: label.to_string(), action, secondary: None, is_valid}
    }

    pub fn double(l1: &str, a1: Action, l2: &str, a2: Action, is_valid: Option<Box<dyn ValidityFn>>) -> Self {
        Bumper::Custom {label: l1.to_string(), action: a1, secondary: Some((l2.to_string(), a2)), is_valid}
    }

    pub fn default(is_valid: Option<Box<dyn ValidityFn>>) -> Self {
        Bumper::Default {is_valid}
    }
}