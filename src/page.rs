use ramp::prism;
use pelican_ui::event::{OnEvent, TickEvent, Event};
use pelican_ui::drawable::{Component, Drawable, SizedTree};
use pelican_ui::{drawables, Context, Callback};
use pelican_ui::layout::{Stack, Offset};
use pelican_ui::canvas::Align;
use pelican_ui::utils::{ValidationFn};
use pelican_ui::interface::general::{Header, Content, Bumper as PelicanBumper, Page as PelicanPage};
use pelican_ui::navigation::AppPage;
use pelican_ui::components::text::{TextSize, TextStyle};
use pelican_ui::theme::Theme;

use std::fmt::Debug;

use crate::{ChkBuilder};
use crate::flow::{Flow, FlowStorageObject};
use crate::items::{Action, Input, Display};
use crate::closure::{NavFn, ScreenBuilder, PageBuilder, SuccessClosure, ReviewItemGetter, SuccessGetter};

pub struct Root;
impl Root {
    pub fn new(title: &str, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), bumper_b: Option<(String, Flow)>) -> PageType {
        PageType::root(title, items, header, bumper_a, bumper_b)
    }
}

#[derive(Debug, Component, Clone)]
pub struct Screen(Stack, pub Box<dyn AppPage>, #[skip] Box<dyn PageBuilder>, #[skip] Option<NavFn>, #[skip] usize, #[skip] ChkBuilder);
impl AppPage for Screen {}
impl OnEvent for Screen {}

impl Screen {
    pub fn new(ctx: &mut Context, builder: &ChkBuilder, mut page_builder: Box<dyn PageBuilder>) -> Self {
        Screen(Stack::default(), ((page_builder)(builder)).build(ctx, builder), page_builder, None, 1, builder.clone())
    }

    pub fn update(&mut self, ctx: &mut Context, new_len: usize, new_fn: Option<NavFn>) {
        println!("Setting screen new_fn to {:?}", new_fn);
        let builder = &self.5;
        self.3 = new_fn.clone();
        self.4 = new_len;
        let mut page_type = (self.2)(&builder);
        if let Some(l) = page_type.length() { *l = self.4; }
        if let Some(nav) = page_type.nav_fn() {
            *nav = self.3.clone();
        }
        self.1 = page_type.build(ctx, &builder);
    }

    pub fn new_builder(builder: &ChkBuilder, page_builder: Box<dyn PageBuilder>) -> Box<dyn ScreenBuilder> {
        let builder = builder.clone();
        Box::new(move |ctx: &mut Context| Screen::new(ctx, &builder.clone(), page_builder.clone())) as Box<dyn ScreenBuilder>
    }
}

#[derive(Clone)]
pub enum PageType {
    Root {title: String, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), bumper_b: Option<(String, Flow)>},
    Display{title: String, items: Vec<Display>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize},
    Input{title: String, item: Input, header: Option<(String, Flow)>, bumper: Bumper, flow_len: usize, next: Option<NavFn>},
    Form{title: String, item: Input, flow_len: usize, next: Option<NavFn>},
    Review{title: String, getter: Box<dyn ReviewItemGetter>, flow_len: usize, next: Option<NavFn>},
    Success{title: String, getter: Box<dyn SuccessGetter>, flow_len: usize}
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

    pub fn form(title: &str, item: Input) -> Self {
        PageType::Form { title: title.to_string(), item, flow_len: 1, next: None }
    }

    pub fn review(title: &str, getter: Box<dyn ReviewItemGetter>) -> Self {
        PageType::Review { title: title.to_string(), getter, flow_len: 1, next: None }
    }

    pub fn success(title: &str, getter: Box<dyn SuccessGetter>) -> Self {
        PageType::Success { title: title.to_string(), getter, flow_len: 1 }
    }

    pub fn nav_fn(&mut self) -> Option<&mut Option<NavFn>> {
        match self {
            PageType::Root{..} |
            PageType::Success{..} => None,
            PageType::Display{next, ..} |
            PageType::Input{next, ..} |
            PageType::Form{next, ..} |
            PageType::Review{next, ..} => Some(next),
        }
    }

    pub fn length(&mut self) -> Option<&mut usize> {
        match self {
            PageType::Root{..} => None,
            PageType::Display{flow_len, ..} |
            PageType::Input{flow_len, ..} |
            PageType::Form{flow_len, ..} |
            PageType::Success{flow_len, ..} |
            PageType::Review{flow_len, ..} => Some(flow_len)
        }
    }

    pub fn build(&self, ctx: &mut Context, builder: &ChkBuilder) -> Box<dyn AppPage> {
        match self {
            PageType::Root{title, items, header, bumper_a, bumper_b} => Box::new(RootPage::new(builder, title.to_string(), items.to_vec(), header.clone(), bumper_a.clone(), bumper_b.clone())),
            PageType::Display{title, items, offset, header, bumper, next, flow_len} => Box::new(StackPage::display(ctx, builder, title.to_string(), items.to_vec(), *offset, header.clone(), bumper.clone(), next.clone(), *flow_len)),
            PageType::Input{title, item, header, bumper, next, flow_len} => Box::new(StackPage::input(ctx, builder, title.to_string(), item.clone(), header.clone(), bumper.clone(), next.clone(), *flow_len)),
            PageType::Form{title, item, next, flow_len} => Box::new(FormPage::new(builder, title.to_string(), item.clone(), next.clone(), *flow_len)),
            PageType::Review{title, getter, next, flow_len} => Box::new(ReviewPage::new(builder, title.to_string(), getter.clone(), next.clone(), *flow_len)),
            PageType::Success{title, getter, flow_len} => Box::new(SuccessPage::new(builder, title.to_string(), getter.clone(), *flow_len)),
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct RootPage(Stack, PelicanPage);
impl OnEvent for RootPage {}
impl AppPage for RootPage {}
impl RootPage {
    pub fn new(builder: &ChkBuilder, title: String, items: Vec<Display>, header: Option<(String, Flow)>, bumper_a: (String, Flow), mut bumper_b: Option<(String, Flow)>) -> Self {
        let theme: &Theme = builder.theme();
        let header_icon = header.as_ref().map(|(s, flow)| {
            let mut flow = flow.clone();
            (s.to_string(), Box::new(move |ctx: &mut Context, theme: &Theme| (flow.build(ctx))(ctx, theme)) as Box<dyn Callback>) 
        });

        let header = Header::home(&theme, &title, header_icon);
        let content = items.clone().iter_mut().filter_map(|di| di.build(builder)).flatten().collect::<Vec<Box<dyn Drawable>>>();
        let second = bumper_b.as_mut().map(|(t, flow)| {
            let mut flow = flow.clone();
            (t.to_string(), Box::new(move |ctx: &mut Context, theme: &Theme| (flow.build(ctx))(ctx, theme)) as Box<dyn Callback>)
        });

        let (title, mut flow) = bumper_a.clone();
        let first = (title.to_string(), Box::new(move |ctx: &mut Context, theme: &Theme| (flow.build(ctx))(ctx, theme)) as Box<dyn Callback>);
        let bumper = PelicanBumper::home(&theme, first, second);

        let offset = match items.first() {
            Some(Display::List {items, ..}) if items.is_empty() => Offset::Center,
            Some(Display::List {..}) => Offset::Start,
            _ if content.len() <= 1 => Offset::Center,
            _ => Offset::Start,
        };

        let page = PelicanPage::new(header, Content::new(offset, content, Box::new(|c| true)), Some(bumper));
        RootPage(Stack::default(), page)
    }
}

#[derive(Debug, Component, Clone)]
pub struct StackPage(Stack, PelicanPage);
impl OnEvent for StackPage {}
impl AppPage for StackPage {}
impl StackPage {
    #[allow(clippy::too_many_arguments)]
    pub fn display(ctx: &mut Context, builder: &ChkBuilder, title: String, items: Vec<Display>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let items = items.into_iter().filter_map(|mut di| di.build(builder)).flatten().collect::<Vec<Box<dyn Drawable>>>();
        StackPage::new(ctx, builder, title, items, offset, header, bumper, next, flow_len)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn input(ctx: &mut Context, builder: &ChkBuilder, title: String, item: Input, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let item = item.build(builder).into_iter().flatten().collect();
        StackPage::new(ctx, builder, title, item, Offset::Start, header, bumper, next, flow_len)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(ctx: &mut Context, builder: &ChkBuilder, title: String, items: Vec<Box<dyn Drawable>>, offset: Offset, header: Option<(String, Flow)>, bumper: Bumper, next: Option<NavFn>, flow_len: usize) -> Self {
        let theme: &Theme = builder.theme();
        let icon = header.map(|(i, mut f)| (i.to_string(), f.build(ctx)));
        let (header, bumper) = match bumper {
            Bumper::Custom {label, action, secondary} => {
                let on_click = action.clone();
                let secondary = secondary.clone().map(|(l, a)| (l, Box::new(move |ctx: &mut Context, theme: &Theme| (a.clone().get())(ctx, theme)) as Box<dyn Callback>));
                let action = Box::new(move |ctx: &mut Context, theme: &Theme| (on_click.clone().get())(ctx, theme));
                let bumper = PelicanBumper::stack(&theme, Some(&label), action, secondary);
                let header = Header::stack(&theme, &title, icon);
                (header, Some(bumper))
            },
            Bumper::Default => match next {
                Some(n) => {
                    let next = n.clone();
                    let bumper = PelicanBumper::stack(&theme, None, Box::new(move |ctx: &mut Context, theme: &Theme| (next.borrow_mut())(ctx, theme)), None);
                    let header = Header::stack(&theme, &title, icon);
                    (header, Some(bumper))
                }
                None => (Header::stack_end(&theme, &title), Some(PelicanBumper::stack_end(&theme, Some(flow_len))))
            },
            Bumper::Done => (Header::stack_end(&theme, &title), Some(PelicanBumper::stack_end(&theme, Some(flow_len)))),
            Bumper::None => (Header::stack(&theme, &title, icon), None),
        };

        let page = PelicanPage::new(header, Content::new(offset, items, Box::new(|c| true)), bumper);
        StackPage(Stack::default(), page)
    }
}

#[derive(Debug, Clone)]
pub enum Bumper {
    Default,
    Custom { label: String, action: Action, secondary: Option<(String, Action)>},
    Done,
    None,
}

impl Bumper {
    pub fn custom(label: &str, action: Action) -> Self {
        Bumper::Custom {label: label.to_string(), action, secondary: None}
    }

    pub fn double(l1: &str, a1: Action, l2: &str, a2: Action) -> Self {
        Bumper::Custom {label: l1.to_string(), action: a1, secondary: Some((l2.to_string(), a2))}
    }
}

#[derive(Debug, Component, Clone)]
pub struct FormPage(Stack, pub PelicanPage);
impl OnEvent for FormPage {}
impl AppPage for FormPage {}
impl FormPage {
    pub fn new(builder: &ChkBuilder, title: String, item: Input, next: Option<NavFn>, flow_len: usize) -> Self {
        println!("Creating form page");
        use pelican_ui::components::TextInput;

        let theme: &Theme = builder.theme();
        let header = Header::stack(&theme, &title, None);
        let bumper = {
            let stack_closure: Box<dyn Callback> = match next {
                Some(n) => {
                    let next = n.clone();
                    Box::new(move |ctx: &mut Context, theme: &Theme| {
                        println!("FormPAGE Next");
                        (next.borrow_mut())(ctx, theme);
                    }) as Box<dyn Callback>
                },
                None => {
                    Box::new(move |ctx: &mut Context, _: &Theme| {println!("Doing nothing")}) as Box<dyn Callback>
                }
            };

            Some(PelicanBumper::stack(&theme, None, stack_closure, None))
        };

        let content = item.build(builder).unwrap_or_default();

        let page = PelicanPage::new(
            header, 
            Content::new(Offset::Start, content, Box::new(|children| {
                let mut result = true;
                children.iter().for_each(|c|
                    if let Some(input) = (*c).as_any().downcast_ref::<TextInput>() {
                        result = !input.value().is_empty();
                    }
                );

                result
            })), 
            bumper
        );

        FormPage(Stack::default(), page)
    }
}

#[derive(Debug, Component, Clone)]
pub struct ReviewPage(Stack, pub PelicanPage, #[skip] Box<dyn ReviewItemGetter>, #[skip] ChkBuilder);
impl OnEvent for ReviewPage {}
impl AppPage for ReviewPage {}
impl ReviewPage {
    pub fn new(builder: &ChkBuilder, title: String, item_getter: Box<dyn ReviewItemGetter>, next: Option<NavFn>, flow_len: usize) -> Self {
        println!("Creating review page");
        use pelican_ui::components::TextInput;

        let theme: &Theme = builder.theme();
        let header = Header::stack(&theme, &title, None);
        let bumper = {
            let stack_closure: Box<dyn Callback> = match next {
                Some(n) => {
                    let next = n.clone();
                    Box::new(move |ctx: &mut Context, theme: &Theme| {
                        println!("ReviewPage Next");
                        (next.borrow_mut())(ctx, theme);
                    }) as Box<dyn Callback>
                },
                None => {
                    Box::new(move |ctx: &mut Context, _: &Theme| {println!("Doing nothing")}) as Box<dyn Callback>
                }
            };

            Some(PelicanBumper::stack(&theme, None, stack_closure, None))
        };

        let page = PelicanPage::new(
            header, 
            Content::new(Offset::Start, Vec::new(), Box::new(|children| true)), 
            bumper
        );

        ReviewPage(Stack::default(), page, item_getter, builder.clone())
    }

    pub fn on_change(&mut self, new: Vec<FlowStorageObject>) {
        let builder = self.3.clone();
        let items: Vec<Display> = (self.2)(new);
        let content = items.into_iter().map(|mut i| i.build(&builder)).flatten().flatten().collect::<Vec<Box<dyn Drawable>>>();
        self.1.content = Content::new(Offset::Start, content, Box::new(|children| true));
    }
}

#[derive(Debug, Component, Clone)]
pub struct SuccessPage(Stack, pub PelicanPage, #[skip] Box<dyn SuccessGetter>, #[skip] ChkBuilder);
impl OnEvent for SuccessPage {}
impl AppPage for SuccessPage {}
impl SuccessPage {
    pub fn new(builder: &ChkBuilder, title: String, getter: Box<dyn SuccessGetter>, flow_len: usize) -> Self {
        println!("Creating success page");

        let theme: &Theme = builder.theme();
        let header = Header::stack_end(&theme, &title);
        let bumper = Some(PelicanBumper::stack_end(&theme, Some(flow_len)));
        let page = PelicanPage::new(
            header, 
            Content::new(Offset::Center, vec![], Box::new(|children| true)), 
            bumper
        );

        SuccessPage(Stack::default(), page, getter, builder.clone())
    }

    pub fn on_change(&mut self, new: Vec<FlowStorageObject>) {
        use pelican_ui::colors;
        use pelican_ui::components::{text::Text, Icon};
        let builder = self.3.clone();
        let theme: &Theme = builder.theme();
        let (icon, description) = (self.2)(new);
        self.1.content = Content::new(Offset::Center, drawables![
            Icon::new(&theme, &icon, Some(theme.colors().get(colors::Text::Heading)), 128.0),
            Text::new(&theme, &description, TextSize::H4, TextStyle::Heading, Align::Center, None)
        ], Box::new(|children| true));
    }
}
