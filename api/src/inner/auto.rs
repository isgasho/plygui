use crate::callbacks::*;

use super::application::Application;
use super::control::Control;

use std::any::Any;

#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait HasInner {
    type I: Sized + 'static;

    fn inner(&self) -> &Self::I;
    fn inner_mut(&mut self) -> &mut Self::I;
    fn into_inner(self) -> Self::I;
}

pub trait Spawnable {
    fn spawn() -> Box<dyn Control>;
}
impl<II: Spawnable + 'static, T: HasInner<I = II> + 'static> Spawnable for T {
    fn spawn() -> Box<dyn Control> {
        <<Self as HasInner>::I as Spawnable>::spawn()
    }
}

on!(Frame(&mut dyn Application) -> bool);
