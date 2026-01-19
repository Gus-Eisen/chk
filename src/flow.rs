use pelican_ui::{Context, Request};
use pelican_ui::components::interface::NavigationEvent;
use pelican_ui::utils::Callback;

use crate::page::{Review, SuccessPage, Success, ReviewPage, Page};
use crate::closure::{NavFn, FnMutClone, ScreenBuilder};
use crate::page::Screen;

use std::rc::Rc;
use std::cell::RefCell;

pub trait Form {
    fn inputs(&self) -> Vec<Box<dyn Page>>;
    fn review(&self) -> Option<Box<dyn ReviewPage>> {None}
    fn success(&self) -> Box<dyn SuccessPage>;
    fn on_submit(&self, ctx: &mut Context);
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
        form.inputs().into_iter().for_each(|p| pages.push(Screen::new_builder(p)));
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

