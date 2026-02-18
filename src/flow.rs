use pelican_ui::{Callback, Context, Request};
use pelican_ui::navigation::{NavigationEvent, Flow as PelicanFlow, FlowContainer, AppPage};
// use pelican_ui::utils::Callback;
use pelican_ui::theme::Theme;
use pelican_ui::drawable::{Drawable, Component, SizedTree};
use pelican_ui::layout::Stack;
use pelican_ui::event::OnEvent;
use ramp::prism;
use pelican_ui::interface::general::Page as PelicanPage;
use pelican_ui::event::{Event, TickEvent};

use crate::items::{EnumItem, Input};
use crate::page::{PageType, Bumper, FormPage, ReviewPage, SuccessPage};
use crate::closure::{FormClosure, NavFn, FnMutClone, ScreenBuilder, MutString, PageBuilder, SuccessClosure, ReviewItemGetter, SuccessGetter};
use crate::page::Screen;
use crate::ChkBuilder;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

// #[derive(Debug, Component)]
// pub enum ChkItem {
//     Root {
//         page: PelicanPage,
//         #[skip] redraw: Box<dyn Redraw>,
//         #[skip] builder: Box<dyn PageBuilder>,
//     }, // This is simply a page that is the start to a tab (bitcoin home, messages home, etc)
//     Form {
//         pages: Enum, // enumerator of all the pages. switching is triggered by a special FormEvent
//         #[skip] storage: HashMap<String, String>, // storage of information collected throughout the form
//         #[skip] history: Vec<Box<dyn Drawable>>
//     }, // this is a flow that creates a form (collects information, displays the result, on_submit)
//     // Display, // displays information (view transaction)
//     // Settings, // allows the change of information over a single page (profile, settings)
// }

// impl OnEvent for ChkItem {
//     fn on_event(&mut self, ctx: &mut Context, size: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
//         if let Some(e) = event.downcast_ref::<FormEvent>() { 
//             match e {
//                 FormEvent::Next()
//             }
//             match self {
//                 ChkItem::Form {pages, storage} => {
                    
//                 }
//             }
//         }
//         vec![event]
//     }
// }

// impl ChkItem {
//     pub fn new_root() -> Self {todo!()}
//     pub fn new_form(items: Vec<FormItem>) -> Self {
//         let first = items[0].title();
//         let pages = items.into_iter().map(|item| (item.title(), Box::new(FormPage::new(item.title(), item.build())) as Box<dyn Drawable>)).collect::<Vec<_>>();
//         let enumerator = Enum::new(pages, first);
//     }

//     pub fn push(&mut self, page: Box<dyn AppPage>) {
//         if let Some(old) = self.pages.right().replace(page) { 
//             self.history.push(old);
//         }
//         self.pages.display_left(false);
//     }

//     pub fn pop(&mut self) {
//         if self.pages.right().is_some() {
//             match self.history.pop() {
//                 Some(page) => *self.pages.right() = Some(page),
//                 None => self.root(None)
//             }
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Form {
    builder: ChkBuilder,
    inputs: Vec<FormItem>,
    review: Review,
    success: Success,
    on_submit: Box<dyn FnMutClone>,
    // storage: FormStorage,   
}

impl Form {
    pub fn new(builder: &ChkBuilder, inputs: Vec<FormItem>, review: Review, success: Success, on_submit: Box<dyn FnMutClone>) -> Self {
        Form {
            inputs,
            builder: builder.clone(),
            review,
            success,
            on_submit,
            // storage: FormStorage::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Success {
    title: String,
    getter: Box<dyn SuccessGetter>,
}

impl Success {
    pub fn new(title: &str, getter: impl SuccessGetter + 'static) -> Self {
        Success{title: title.to_string(), getter: Box::new(getter)}
    }
}

#[derive(Debug, Clone)]
pub struct Review {
    title: String,
    getter: Box<dyn ReviewItemGetter>,
}

impl Review {
    pub fn new(title: &str, getter: impl ReviewItemGetter + 'static) -> Self {
        Review{title: title.to_string(), getter: Box::new(getter)}
    }
}

#[derive(Debug, Clone)]
pub enum FormItem {
    Text(String, Box<dyn FormClosure>),
    Number(String, NumberVariant, Box<dyn FormClosure>),
    Enum(String, Vec<EnumItem>),
}

pub struct FormStorage(pub HashMap<String, String>);

impl FormItem {
    pub fn text(text: &str) -> Self {
        let text = text.to_string();
        FormItem::Text(text.to_string(), Box::new(move |storage: &mut FormStorage, value: String| {storage.0.insert(text.to_string(), value);}))
    }

    pub fn number(title: &str, number: NumberVariant) -> Self {
        FormItem::Number(title.to_string(), number, Box::new(|_, _| println!("Nothing doing")))
    }

    pub fn enumerator(label: &str, items: Vec<(&str, &str)>) -> Self {
        let items = items.into_iter().map(|(a, b)| {
            EnumItem::new(a, b)
        }).collect::<Vec<EnumItem>>();
        FormItem::Enum(label.to_string(), items)
    }
}

impl FormItem {
    fn title(&self) -> String {
        match self {
            FormItem::Text(title, ..) |
            FormItem::Number(title, ..) |
            FormItem::Enum(title, ..) => title.to_string()
        }
    }
    fn build(&self) -> Input {
        match self {
            FormItem::Text(title, getter) => {
                let title = title.clone();
                Input::text(&title, false, None, None)
            },
            FormItem::Number(title, variant, getter) => {
                match variant {
                    NumberVariant::Currency => Input::currency("Enter dollar amount"),
                    NumberVariant::Date => Input::date("Enter date"),
                    NumberVariant::Time => Input::time("Enter time"),
                }
            },
            FormItem::Enum(title, items) => {
                Input::enumerator(items.clone())
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Flow(Vec<Box<dyn ScreenBuilder>>, Option<Box<dyn FnMutClone>>);
impl Flow{
    pub fn new(pages: Vec<Box<dyn ScreenBuilder>>) -> Self {
        Flow(pages, None)
    }

    pub fn from_form(form: Form) -> Self {
        let builder = form.builder;
        let mut pages: Vec<Box<dyn ScreenBuilder>> = vec![];
        form.inputs.into_iter().for_each(|input| {
            let input = Box::new(move |builder: &ChkBuilder| {
                PageType::form(&input.title(), input.build())
            }) as Box<dyn PageBuilder>;
            pages.push(Screen::new_builder(&builder, input))
        });

        let review = form.review;
        let review = Box::new(move |builder: &ChkBuilder| {
            let review = review.clone();
            PageType::review(&review.title, review.getter)
        }) as Box<dyn PageBuilder>;
        pages.push(Screen::new_builder(&builder, review));

        let success = form.success;
        let success = Box::new(move |builder: &ChkBuilder| {
            let success = success.clone();
            PageType::success(&success.title, success.getter)
        }) as Box<dyn PageBuilder>;
        pages.push(Screen::new_builder(&builder, success));

        // if let Some(r) = form.review() {pages.push(Screen::new_builder(builder, Review(r)));}
        // pages.push(Screen::new_builder(builder, Success(form.success())));
        let mut on_submit = form.on_submit.clone();
        let on_submit: Box<dyn FnMutClone> = Box::new(move |ctx: &mut Context| (on_submit)(ctx));
        Flow(pages, Some(on_submit))
    }
    
    pub(crate) fn build(&mut self, ctx: &mut Context) -> Box<dyn Callback> {
        let mut new: Vec<Box<dyn AppPage>> = vec![];
        let length = self.0.len();
        if self.0.is_empty() { return Box::new(|_ctx, _| {}); }

        let mut pages = self.0.clone();
        let mut first = pages.remove(0);
        let mut next_fn: Option<NavFn> = None;

        for (i, mut page) in pages.into_iter().rev().enumerate() {
            let callback = (i == 0).then_some(self.1.clone()).flatten(); 
            let mut page: Screen = (page)(ctx);
            page.update(ctx, length, next_fn.take());
            new.push(Box::new(page));
            next_fn = Some(NavFn(Rc::new(RefCell::new(move |ctx: &mut Context, _: &Theme| {
                if let Some(cb) = callback.clone() { (cb.clone())(ctx) } // on_submit
                ctx.send(Request::event(NavigationEvent::Next));
            }))));
        }

        let mut first = (first)(ctx);
        first.update(ctx, length, next_fn.clone());
        new.push(Box::new(first));
        new.reverse();


        Box::new(move |ctx: &mut Context, _: &Theme| {
            let flow = FlowWrapper::new(PelicanFlow::new(new.clone()));
            ctx.send(Request::event(NavigationEvent::push(flow))); // this needs to push the flow
        })
    }
}

// need to add a review page that takes in the vector of storage objects.
// need to add a success page. 
// need to add a on_submit function

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FlowStorageObject {
    TextInput(String),
    NumericalInput(String),
    RadioSelector(usize)
}


#[derive(Debug, Component, Clone)]
pub struct FlowWrapper(Stack, PelicanFlow, #[skip] Vec<FlowStorageObject>);
impl OnEvent for FlowWrapper {
    fn on_event(&mut self, ctx: &mut Context, sized: &SizedTree, event: Box<dyn Event>) -> Vec<Box<dyn Event>> {
        use pelican_ui::components::{TextInput, NumericalInput, RadioSelector};
        
        if event.downcast_ref::<TickEvent>().is_some() {
            
            if let Some(page) = self.1.current.as_mut().unwrap().downcast_mut::<Screen>().as_mut().unwrap().1.downcast_mut::<ReviewPage>() {
                page.on_change(self.2.clone());
            } else if let Some(page) = self.1.current.as_mut().unwrap().downcast_mut::<Screen>().as_mut().unwrap().1.downcast_mut::<SuccessPage>() {
                page.on_change(self.2.clone());
            } else {
                let mut all = self.1.stored.clone();
                all.insert(self.1.index, self.1.current.clone().unwrap());
                self.2 = Vec::new();
                for each in all {
                    if let Some(page) = each.downcast_ref::<Screen>().unwrap().1.downcast_ref::<FormPage>() {
                        let children = page.1.content.children();
                        for child in children {
                            if let Some(input) = child.downcast_ref::<TextInput>() {
                                self.2.push(FlowStorageObject::TextInput(input.value()));
                            } else if let Some(input) = child.downcast_ref::<NumericalInput>() {
                                self.2.push(FlowStorageObject::NumericalInput(input.value()));
                            } else if let Some(selector) = child.downcast_ref::<RadioSelector>() {
                                self.2.push(FlowStorageObject::RadioSelector(selector.value()));
                            }
                        }
                    }
                }
            }
        }
        vec![event]
    }
}

impl FlowWrapper {
    fn new(flow: PelicanFlow) -> Self {Self(Stack::default(), flow, vec![])}
}

impl FlowContainer for FlowWrapper {
    fn flow(&mut self) -> &mut PelicanFlow {&mut self.1}
}

// pub struct Flow(Flow, Storage);
// impl Flow {
//     fn new(pages) -> Self {
//         let mut new = Vec::new();
//         for page in pages {
//             if let Some(cb) = callback.clone() { (cb.clone())(ctx) } // on_submit
//             let mut page: Screen = (page)(ctx);
//             page.update(ctx, length, next.clone());
//             new.push(page);
//         }
//         Flow::new(new)
//     }
// }

// impl FlowContainer for Flow {
//     fn flow(&mut self) -> &mut Flow {&mut self.0}
// }

#[derive(Clone, Debug)]
pub enum NumberVariant {
    Currency,
    Date,
    Time,
}
