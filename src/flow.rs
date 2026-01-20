use pelican_ui::{Context, Request};
use pelican_ui::components::interface::NavigationEvent;
use pelican_ui::utils::Callback;

use crate::items::{EnumItem, Input};
use crate::page::{Review, SuccessPage, Success, ReviewPage, Page, PageType, Bumper};
use crate::closure::{NavFn, FnMutClone, ScreenBuilder, MutString, PageBuilder, SuccessClosure};
use crate::page::Screen;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum NumberVariant {
    Currency(u32),
    Date,
    Time,
}

pub trait Form {
    fn inputs(&self) -> Vec<FormItem>;
    fn review(&self) -> Option<Box<dyn ReviewPage>> {None}
    fn success(&self) -> Box<dyn SuccessClosure>;
    fn on_submit(&self, ctx: &mut Context) {}
}

pub enum FormItem {
    Text(String, Box<dyn MutString>),
    Number(String, NumberVariant, Box<dyn MutString>),
    Enum(String, Vec<EnumItem>),
}

impl FormItem {
    pub fn text(text: &str, closure: impl FnMut(&mut Context) -> &mut String + Clone + 'static) -> Self {
        FormItem::Text(text.to_string(), Box::new(closure))
    }

    pub fn number(title: &str, number: NumberVariant, closure: impl FnMut(&mut Context) -> &mut String + Clone + 'static) -> Self {
        FormItem::Number(title.to_string(), number, Box::new(closure))
    }

    pub fn enumerator(label: &str, items: Vec<EnumItem>) -> Self {
        FormItem::Enum(label.to_string(), items)
    }
}

impl FormItem {
    fn build(&self) -> Box<dyn Page> {
        match self {
            FormItem::Text(title, getter) => {
                let title = title.clone();
                let getter = getter.clone();
                Box::new(FormPage(Box::new(move |ctx: &mut Context| {
                    let mut getter = getter.clone();
                    let setter = getter.clone();
                    let preset = Some(getter(ctx).clone()).filter(|s| !s.is_empty());
                    PageType::input(&title, 
                        Input::text(&title, false, preset, None, move |ctx: &mut Context, val: &mut String| {
                            *(setter.clone())(ctx) = val.to_string();
                        }), None,
                        Bumper::default(Some(Box::new(move |ctx: &mut Context| (getter.clone())(ctx).is_empty())))
                    )
                })))
            }
            FormItem::Number(title, variant, getter) => {
                let variant = variant.clone();
                let title = title.clone();
                let getter = getter.clone();
                Box::new(FormPage(Box::new(move |ctx: &mut Context| {
                    let mut getter = getter.clone();
                    let setter = getter.clone();

                    let input = match variant {
                        NumberVariant::Currency(val) => {
                            Input::currency("Enter dollar amount", move |ctx: &mut Context, val: &mut String| {
                                *(setter.clone())(ctx) = val.to_string();
                            })
                        }
                        NumberVariant::Date => panic!("Not accepting date variant yet"),
                        NumberVariant::Time => panic!("Not accepting time variant yet")
                    };

                    PageType::input(&title, input, None, Bumper::default(Some(Box::new(move |ctx: &mut Context| (getter.clone())(ctx).is_empty()))))
                })))
            },
            FormItem::Enum(title, items) => {
                let title = title.clone();
                let items = items.clone();
                Box::new(FormPage(Box::new(move |ctx: &mut Context| {
                    PageType::input(&title, Input::enumerator(items.clone()), None, Bumper::default(Some(Box::new(move |ctx: &mut Context| false))))
                })))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormPage(Box<dyn PageBuilder>);
impl Page for FormPage {
    fn page(&mut self, ctx: &mut Context) -> Box<dyn PageBuilder> {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Flow(Vec<Box<dyn ScreenBuilder>>, Option<Box<dyn FnMutClone>>);
impl PartialEq for Flow {
    fn eq(&self, other: &Self) -> bool {self.0.len() == other.0.len()}
}
impl Flow{
    pub fn new(pages: Vec<Box<dyn ScreenBuilder>>) -> Self {
        Flow(pages, None)
    }

    pub fn from_form(form: impl Form + 'static + Clone) -> Self {
        let mut pages: Vec<Box<dyn ScreenBuilder>> = vec![];
        form.inputs().into_iter().for_each(|input| {
            let input: Box<dyn Page> = input.build();
            pages.push(Screen::new_builder(input))
        });
        if let Some(r) = form.review() {pages.push(Screen::new_builder(Review(r)));}
        pages.push(Screen::new_builder(Success(form.success())));
        let on_submit: Box<dyn FnMutClone> = Box::new(move |ctx: &mut Context| form.on_submit(ctx));
        Flow(pages, Some(on_submit))
    }
    
    pub(crate) fn build(&mut self) -> Callback {
        let length = self.0.len();
        if self.0.is_empty() { return Box::new(|_ctx| {}); }

        let mut pages = self.0.clone();
        let mut first = pages.remove(0);
        let mut next_fn: Option<NavFn> = None;

        for (i, page) in pages.into_iter().rev().enumerate() {
            let callback = (i == 0).then_some(self.1.clone()).flatten(); 
            let next = next_fn.take();
            let mut page = page;
            next_fn = Some(NavFn(Rc::new(RefCell::new(move |ctx: &mut Context| {
                if let Some(cb) = callback.clone() { (cb.clone())(ctx) }
                let mut page: Screen = (page)(ctx);
                page.update(ctx, length, next.clone());
                ctx.send(Request::event(NavigationEvent::push(page)));
            }))));
        }

        Box::new(move |ctx: &mut Context| {
            let mut page = (first)(ctx);
            page.update(ctx, length, next_fn.clone());
            ctx.send(Request::event(NavigationEvent::push(page)));
        })
    }
}

