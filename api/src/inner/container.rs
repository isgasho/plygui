use crate::{types};

use super::auto::{HasInner};
use super::control::{Control, ControlInner, AControl};
use super::member::{Member, AMember, MemberInner};

define! {
    Container: Member {
        outer: {
            fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
            fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;
        },
        inner: {
            fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
            fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;
        }
    }
}

impl<T: ContainerInner> Container for AMember<T> {
    #[inline]
    default fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner.find_control(arg)
    }

    #[inline]
    default fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    default fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    default fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}
impl<T: ContainerInner + ControlInner> Container for AMember<AControl<AContainer<T>>> {
    #[inline]
    default fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.inner.find_control(arg)
    }

    #[inline]
    default fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    default fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    default fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}
impl<T: ContainerInner> MaybeContainer for AMember<AContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: ContainerInner + ControlInner> MaybeContainer for AMember<AControl<AContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<II: ContainerInner, T: HasInner<I=II> + 'static> ContainerInner for T {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner_mut().find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner().find_control(arg)
    }
}
