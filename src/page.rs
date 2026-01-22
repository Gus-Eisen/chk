use ramp::prism;
use pelican_ui::event::{OnEvent, TickEvent, Event};
use pelican_ui::drawable::{Component, Drawable, SizedTree};
use pelican_ui::Context;
use pelican_ui::layout::{Stack, Offset};
use pelican_ui::canvas::Align;
use pelican_ui::utils::{Callback, ValidationFn};
use pelican_ui::components::interface::{Header, Content, Bumper as PelicanBumper, Page as PelicanPage, AppPage};
use pelican_ui::components::text::{TextSize, TextStyle};

use std::fmt::Debug;

use crate::flow::Flow;
use crate::items::{Action, Input, Display};
use crate::closure::{NavFn, ScreenBuilder, PageBuilder, RootBuilder, SuccessClosure};

pub struct RootP(Box<dyn PageBuilder>);
impl RootP {
    pub fn new(title: &str, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), bumper_b: Option<(String, Flow)>) -> Self {
        let root = PageType::root(title, items, header, bumper_a, bumper_b);
        RootP(Box::new(move |_: &mut Context| {root.clone()}))
    }
}

#[derive(Clone, Debug)]
pub struct Success(pub Box<dyn SuccessClosure>);

impl Page for Success {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        let [title, icon, text] = (self.0)(ctx);

        Box::new(move |_: &mut Context| {
            let items = vec![Display::icon(&icon.clone()), Display::Text {text: text.clone(), size: TextSize::H4, style: TextStyle::Heading, align: Align::Center}];
            PageType::display(&title.clone(), items, None, Bumper::Done, Offset::Center)
        })
    }
}

pub trait ReviewPage: Debug + dyn_clone::DynClone {
    // TODO: Needs to return the specific Display Items cause currently they could make the review page an input screen (BAD)
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder>;
}

dyn_clone::clone_trait_object!(ReviewPage);

#[derive(Debug, Clone)]
pub struct Review(pub Box<dyn ReviewPage>);
impl Review {
    pub fn from(page: impl ReviewPage + Clone + 'static) -> Self {Review(Box::new(page))}
}

impl Page for Review {
    fn page(&mut self, _ctx: &mut Context) -> Box<dyn PageBuilder> {
        let mut review = self.0.clone();
        Box::new(move |ctx: &mut Context| {ReviewPage::page(&mut *review, ctx)(ctx)})
    }
}

pub trait Root: Debug {
    fn page(&mut self, _ctx: &mut Context) -> Box<dyn RootBuilder>;
    fn redraw(&mut self, _ctx: &mut Context) -> bool {false}
}

impl<R: Root + dyn_clone::DynClone> Page for R {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        let root = Root::page(self, ctx);
        Box::new(move |ctx: &mut Context| ((root.clone())(ctx)).0)(ctx)
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {Root::redraw(self, ctx)}
}

pub trait Page: Debug + dyn_clone::DynClone {
    fn page(&mut self, _ctx: &mut Context) -> Box<dyn PageBuilder>;
    fn redraw(&mut self, _ctx: &mut Context) -> bool {false}
}

dyn_clone::clone_trait_object!(Page);

impl Page for Box<dyn Page> {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        Page::page(&mut **self, ctx)
    }

    fn redraw(&mut self, ctx: &mut Context) -> bool {
        Page::redraw(&mut **self, ctx)
    }
}

#[derive(Debug, Component)]
pub struct Screen(Stack, Box<dyn Drawable>, #[skip] Box<dyn Page>, #[skip] Option<NavFn>, #[skip] usize);
impl AppPage for Screen {}
impl OnEvent for Screen {
    fn on_event(&mut self, ctx: &mut Context, _sized: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        if event.downcast_ref::<TickEvent>().is_some() && self.2.redraw(ctx) {
            let mut page_type = (self.2.page(ctx))(ctx);
            if let Some(l) = page_type.length() { *l = self.4; }
            if let Some(nav) = page_type.nav_fn() {
                *nav = self.3.clone();
            }
            self.1 = Box::new(page_type.build(ctx));
        }
        vec![event]
    }
}

impl Screen {
    pub fn new(ctx: &mut Context, mut page: impl Page + 'static) -> Self {
        Screen(Stack::default(), Box::new((page.page(ctx))(ctx).build(ctx)), Box::new(page), None, 1)
    }

    pub fn update(&mut self, ctx: &mut Context, new_len: usize, new_fn: Option<NavFn>) {
        self.3 = new_fn.clone();
        self.4 = new_len;
        let mut page_type = (self.2.page(ctx))(ctx);
        if let Some(l) = page_type.length() { *l = self.4; }
        if let Some(nav) = page_type.nav_fn() {
            *nav = self.3.clone();
        }
        self.1 = Box::new(page_type.build(ctx));
    }

    pub fn new_builder(page: impl Page + Clone + 'static) -> Box<dyn ScreenBuilder> {
        Box::new(move |ctx: &mut Context| Screen::new(ctx, page.clone())) as Box<dyn ScreenBuilder>
    }
}

#[derive(Clone)]
pub enum PageType {
    Root {title: String, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), bumper_b: Option<(String, Flow)>},
    Display{title: String, items: Vec<Display>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize},
    Input{title: String, item: Input, header: Option<(String, Flow)>, bumper: Bumper, flow_len: usize, next: Option<NavFn>}
}

impl PageType {
    pub fn root(title: &str, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), bumper_b: Option<(String, Flow)>) -> Self {
        PageType::Root { title: title.to_string(), items, header, bumper_a, bumper_b }
    }

    pub fn display(title: &str, items: Vec<Display>, header: Option<(String, Flow)>, bumper: Bumper, offset: Offset) -> Self {
        PageType::Display { title: title.to_string(), items, header, bumper, offset, flow_len: 1, next: None }
    }

    pub fn input(title: &str, item: Input, header: Option<(String, Flow)>, bumper: Bumper) -> Self {
        PageType::Input { title: title.to_string(), item, header, bumper, flow_len: 1, next: None }
    }

    pub fn nav_fn(&mut self) -> Option<&mut Option<NavFn>> {
        match self {
            PageType::Root{..} => None,
            PageType::Display{next, ..} => Some(next),
            PageType::Input{next, ..} => Some(next)
        }
    }

    pub fn length(&mut self) -> Option<&mut usize> {
        match self {
            PageType::Root{..} => None,
            PageType::Display{flow_len, ..} => Some(flow_len),
            PageType::Input{flow_len, ..} => Some(flow_len),
        }
    }

    pub fn build(&self, ctx: &mut Context) -> Box<dyn Drawable> {
        match self {
            PageType::Root{title, items, header, bumper_a, bumper_b} => Box::new(RootPage::new(ctx, title.to_string(), items.to_vec(), header.clone(), bumper_a.clone(), bumper_b.clone())),
            PageType::Display{title, items, offset, header, bumper, next, flow_len} => Box::new(StackPage::display(ctx, title.to_string(), items.to_vec(), *offset, header.clone(), bumper.clone(), next.clone(), *flow_len)),
            PageType::Input{title, item, header, bumper, next, flow_len} => Box::new(StackPage::input(ctx, title.to_string(), item.clone(), header.clone(), bumper.clone(), next.clone(), *flow_len))
        }
    }
}

#[derive(Debug, Component)]
pub struct RootPage(Stack, PelicanPage);
impl OnEvent for RootPage {}
impl RootPage {
    pub fn new(ctx: &mut Context, title: String, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), mut bumper_b: Option<(String, Flow)>) -> Self {
        let header_icon = header.as_ref().map(|(s, flow)| {
            let mut flow = flow.clone();
            (s.to_string(), Box::new(move |ctx: &mut Context| (flow.build())(ctx)) as Callback) 
        });

        let header = Header::home(ctx, &title, header_icon);
        let content = items.clone().iter_mut().filter_map(|di| di.build(ctx)).flatten().collect::<Vec<Box<dyn Drawable>>>();
        let second = bumper_b.as_mut().map(|(t, flow)| {
            let mut flow = flow.clone();
            (t.to_string(), Box::new(move |ctx: &mut Context| (flow.build())(ctx)) as Callback)
        });

        let (title, mut flow) = bumper_a.clone();
        let first = (title.to_string(), Box::new(move |ctx: &mut Context| (flow.build())(ctx)) as Callback);
        let bumper = PelicanBumper::home(ctx, first, second, None);

        let offset = match items.first() {
            Some(Display::List {items, ..}) if items.is_empty() => Offset::Center,
            Some(Display::List {..}) => Offset::Start,
            _ if content.len() <= 1 => Offset::Center,
            _ => Offset::Start,
        };

        let page = PelicanPage::new(header, Content::new(offset, content), Some(bumper));
        RootPage(Stack::default(), page)
    }
}

#[derive(Debug, Component)]
pub struct StackPage(Stack, PelicanPage);
impl OnEvent for StackPage {}
impl StackPage {
    #[allow(clippy::too_many_arguments)]
    pub fn display(ctx: &mut Context, title: String, items: Vec<Display>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let items = items.into_iter().filter_map(|mut di| di.build(ctx)).flatten().collect::<Vec<Box<dyn Drawable>>>();
        StackPage::new(ctx, title, items, offset, header, bumper, next, flow_len)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn input(ctx: &mut Context, title: String, item: Input, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let item = item.build(ctx).into_iter().flatten().collect();
        StackPage::new(ctx, title, item, Offset::Start, header, bumper, next, flow_len)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, title: String, items: Vec<Box<dyn Drawable>>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let icon = header.map(|(i, mut f)| (i.to_string(), f.build()));
        let (header, bumper) = match bumper {
            Bumper::Custom {label, action, secondary, is_valid} => {
                let on_click = action.clone();
                let secondary = secondary.clone().map(|(l, a)| (l, Box::new(move |ctx: &mut Context| (a.clone().get())(ctx)) as Callback));
                let action = Box::new(move |ctx: &mut Context| (on_click.clone().get())(ctx));
                let validity_fn = is_valid.clone().map(|mut vfn| Box::new(move |ctx: &mut Context| (vfn)(ctx)) as Box<dyn ValidationFn>);
                let bumper = PelicanBumper::stack(ctx, Some(&label), action, secondary, validity_fn);
                let header = Header::stack(ctx, &title, icon);
                (header, Some(bumper))
            },
            Bumper::Default {is_valid} => match next {
                Some(n) => {
                    let validity_fn = is_valid.clone().map(|mut vfn| Box::new(move |ctx: &mut Context| (vfn)(ctx)) as Box<dyn ValidationFn>);
                    let next = n.clone();
                    let bumper = PelicanBumper::stack(ctx, None, Box::new(move |ctx: &mut Context| (next.borrow_mut())(ctx)), None, validity_fn);
                    let header = Header::stack(ctx, &title, icon);
                    (header, Some(bumper))
                }
                None => (Header::stack_end(ctx, &title), Some(PelicanBumper::stack_end(ctx, Some(flow_len))))
            },
            Bumper::Done => (Header::stack_end(ctx, &title), Some(PelicanBumper::stack_end(ctx, Some(flow_len)))),
            Bumper::None => (Header::stack(ctx, &title, icon), None),
        };

        let page = PelicanPage::new(header, Content::new(offset, items), bumper);
        StackPage(Stack::default(), page)
    }
}

#[derive(Debug, Clone)]
pub enum Bumper {
    Default { is_valid: Option<Box<dyn ValidationFn>> },
    Custom { label: String, action: Action, secondary: Option<(String, Action)>, is_valid: Option<Box<dyn ValidationFn>>},
    Done,
    None,
}

impl Bumper {
    pub fn custom(label: &str, action: Action, is_valid: Option<Box<dyn ValidationFn>>) -> Self {
        Bumper::Custom {label: label.to_string(), action, secondary: None, is_valid}
    }

    pub fn double(l1: &str, a1: Action, l2: &str, a2: Action, is_valid: Option<Box<dyn ValidationFn>>) -> Self {
        Bumper::Custom {label: l1.to_string(), action: a1, secondary: Some((l2.to_string(), a2)), is_valid}
    }

    pub fn default(is_valid: Option<Box<dyn ValidationFn>>) -> Self {
        Bumper::Default {is_valid}
    }
}