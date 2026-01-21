#![allow(clippy::type_complexity)]
use pelican_ui::Context;
use crate::page::{Screen, RootP, PageType};

use std::rc::Rc;
use std::cell::RefCell;

pub(crate) trait FnMutClone: FnMut(&mut Context) + 'static {
    fn clone_box(&self) -> Box<dyn FnMutClone>;
}

impl PartialEq for dyn FnMutClone{fn eq(&self, _: &Self) -> bool {true}}

impl<F> FnMutClone for F where F: FnMut(&mut Context) + Clone + 'static {
    fn clone_box(&self) -> Box<dyn FnMutClone> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FnMutClone> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn FnMutClone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clonable Closure")
    }
}

pub(crate) trait ValidityFn: FnMut(&mut Context) -> bool + 'static {
    fn clone_box(&self) -> Box<dyn ValidityFn>;
}

impl<F> ValidityFn for F where F: FnMut(&mut Context) -> bool + Clone + 'static {
    fn clone_box(&self) -> Box<dyn ValidityFn> {
        Box::new(self.clone())
    }
}

impl PartialEq for dyn ValidityFn{fn eq(&self, _: &Self) -> bool {true}}

impl Clone for Box<dyn ValidityFn> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn ValidityFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Valitidy check...")
    }
}

pub(crate) trait EditedFn: FnMut(&mut Context, &mut String) + 'static {
    fn clone_box(&self) -> Box<dyn EditedFn>;

    fn get(&self) -> Box<dyn FnMut(&mut Context, &mut String)> {
        let mut closure = self.clone_box();
        Box::new(move |ctx: &mut Context, val: &mut String| (closure)(ctx, val))
    }
}

impl PartialEq for dyn EditedFn{fn eq(&self, _: &Self) -> bool {true}}

impl<F> EditedFn for F where F: FnMut(&mut Context, &mut String) + Clone + 'static {
    fn clone_box(&self) -> Box<dyn EditedFn> { Box::new(self.clone()) }
}

impl Clone for Box<dyn EditedFn> { fn clone(&self) -> Self { self.as_ref().clone_box() } }

impl std::fmt::Debug for dyn EditedFn { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "EditedFn") } }

pub(crate) type NavFnInner = Rc<RefCell<dyn FnMut(&mut Context)>>;

#[derive(Clone)]
pub(crate) struct NavFn(pub NavFnInner);

impl PartialEq for NavFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl std::ops::Deref for NavFn {
    type Target = NavFnInner;
    fn deref(&self) -> &Self::Target { &self.0 }
}


impl std::fmt::Debug for NavFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NavFN")
    }
}

pub trait PageBuilder: FnMut(&mut Context) -> PageType + 'static {
    fn clone_box(&self) -> Box<dyn PageBuilder>;
}

impl<F> PageBuilder for F where F: FnMut(&mut Context) -> PageType + Clone + 'static {
    fn clone_box(&self) -> Box<dyn PageBuilder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PageBuilder> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn PageBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PageBuilder")
    }
}


pub trait RootBuilder: FnMut(&mut Context) -> RootP + 'static {
    fn clone_box(&self) -> Box<dyn RootBuilder>;
}

impl<F> RootBuilder for F where F: FnMut(&mut Context) -> RootP + Clone + 'static {
    fn clone_box(&self) -> Box<dyn RootBuilder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RootBuilder> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn RootBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RootBuilder")
    }
}


pub trait ScreenBuilder: FnMut(&mut Context) -> Screen + 'static {
    fn clone_box(&self) -> Box<dyn ScreenBuilder>;
}

impl<F> ScreenBuilder for F where F: FnMut(&mut Context) -> Screen + Clone + 'static {
    fn clone_box(&self) -> Box<dyn ScreenBuilder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ScreenBuilder> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn ScreenBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScreenBuilder")
    }
}


pub(crate) trait MutString: FnMut(&mut Context) -> &mut String + 'static {
    fn clone_box(&self) -> Box<dyn MutString>;
}

impl<F> MutString for F where F: FnMut(&mut Context) -> &mut String + Clone + 'static {
    fn clone_box(&self) -> Box<dyn MutString> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MutString> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn MutString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MutString")
    }
}

pub trait SuccessClosure: FnMut(&mut Context) -> [String; 3] + 'static {
    fn clone_box(&self) -> Box<dyn SuccessClosure>;
}

impl<F> SuccessClosure for F where F: FnMut(&mut Context) -> [String; 3] + Clone + 'static {
    fn clone_box(&self) -> Box<dyn SuccessClosure> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SuccessClosure> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl std::fmt::Debug for dyn SuccessClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuccessClosure")
    }
}