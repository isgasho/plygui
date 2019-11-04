use crate::{layout, types};

use super::HasInner;
use super::auto::{HasSizeInner, HasVisibilityInner, HasLayoutInner, MaybeContainer};
use super::control::{Control, ControlInner, AControl, ControlBase};
use super::container::{Container, ContainerInner};
use super::native_id::HasNativeIdInner;
use super::drawable::{Drawable};
use super::member::{Member, MemberBase, AMember, MemberInner};

define! {
    MultiContainer: Container {
        outer: {
            fn len(&self) -> usize;
            fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
            fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>>;
            fn child_at(&self, index: usize) -> Option<&dyn Control>;
            fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control>;

            fn is_empty(&self) -> bool {
                self.len() < 1
            }
            fn clear(&mut self) {
                let len = self.len();
                for index in (0..len).rev() {
                    self.remove_child_from(index);
                }
            }
            fn push_child(&mut self, child: Box<dyn Control>) {
                let len = self.len();
                self.set_child_to(len, child);
            }
            fn pop_child(&mut self) -> Option<Box<dyn Control>> {
                let len = self.len();
                if len > 0 {
                    self.remove_child_from(len - 1)
                } else {
                    None
                }
            }
        }
        inner: {
            fn len(&self) -> usize;
            fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
            fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<dyn Control>>;
            fn child_at(&self, index: usize) -> Option<&dyn Control>;
            fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control>;
        
            #[inline]
            fn is_empty(&self) -> bool {
                self.len() < 1
            }
            #[inline]
            fn clear(&mut self, base: &mut MemberBase) {
                let len = self.len();
                for index in (0..len).rev() {
                    self.remove_child_from(base, index);
                }
            }
            #[inline]
            fn push_child(&mut self, base: &mut MemberBase, child: Box<dyn Control>) {
                let len = self.len();
                self.set_child_to(base, len, child);
            }
            #[inline]
            fn pop_child(&mut self, base: &mut MemberBase) -> Option<Box<dyn Control>> {
                let len = self.len();
                if len > 0 {
                    self.remove_child_from(base, len - 1)
                } else {
                    None
                }
            }
        }
    }
}

impl<T: MultiContainerInner> HasNativeIdInner for AMultiContainer<T> {
    type Id = T::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: MultiContainerInner> MemberInner for AMultiContainer<T> {}

impl<T: MultiContainerInner + ContainerInner> ContainerInner for AMultiContainer<T> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner.find_control(arg)
    }
}
impl<T: MultiContainerInner + ControlInner + Drawable> Drawable for AMultiContainer<T> {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl<T: MultiContainerInner + ControlInner> HasLayoutInner for AMultiContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: MultiContainerInner + ControlInner> HasSizeInner for AMultiContainer<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: MultiContainerInner + ControlInner> HasVisibilityInner for AMultiContainer<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: MultiContainerInner + ControlInner> ControlInner for AMultiContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container) {
        self.inner.on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry) {
        self.inner.fill_from_markup(member, control, markup, registry)
    }
}
impl<T: MultiContainerInner> MultiContainer for AMember<AMultiContainer<T>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>> {
        self.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>> {
        self.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn Control> {
        self.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control> {
        self.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn MultiContainer> {
        self
    }
}
impl<T: MultiContainerInner> MaybeContainer for AMember<AMultiContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner> Container for AMember<AMultiContainer<T>> {
    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&dyn MultiContainer> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> Container for AMember<AControl<AMultiContainer<T>>> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
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
        self.inner.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
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
        self.inner.inner.find_control(arg)
    }

    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&dyn MultiContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}
impl<T: MultiContainerInner + ControlInner> MaybeContainer for AMember<AControl<AMultiContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> MultiContainer for AMember<AControl<AMultiContainer<T>>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn Control> {
        self.inner.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control> {
        self.inner.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn MultiContainer> {
        self
    }
}
