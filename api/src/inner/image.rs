use crate::types;

use super::auto::HasInner;
use super::control::{AControl, Control, ControlBase, ControlInner};
use super::has_image::{HasImage, HasImageInner};
use super::member::{AMember, MemberBase, MemberInner};

define! {
    Image: Control + HasImage {
        outer: {
            fn set_scale(&mut self, policy: types::ImageScalePolicy);
            fn scale(&self) -> types::ImageScalePolicy;
        },
        inner: {
            fn with_content(content: image::DynamicImage) -> Box<dyn Image>;
            fn set_scale(&mut self, member: &mut MemberBase, control: &mut ControlBase, policy: types::ImageScalePolicy);
            fn scale(&self) -> types::ImageScalePolicy;
        }
    }
}

impl<II: ImageInner, T: HasInner<I = II> + 'static> ImageInner for T {
    fn with_content(content: image::DynamicImage) -> Box<dyn Image> {
        <<Self as HasInner>::I as ImageInner>::with_content(content)
    }
    fn set_scale(&mut self, member: &mut MemberBase, control: &mut ControlBase, policy: types::ImageScalePolicy) {
        self.inner_mut().set_scale(member, control, policy)
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.inner().scale()
    }
}

impl<T: ImageInner> Image for AMember<AControl<AImage<T>>> {
    fn set_scale(&mut self, policy: types::ImageScalePolicy) {
        let base1 = self as *mut _ as *mut AMember<AControl<AImage<T>>>;
        let base2 = self as *mut _ as *mut AMember<AControl<AImage<T>>>;
        self.inner.inner.inner.set_scale(&mut (unsafe { (&mut *base1) }.base), &mut (unsafe { (&mut *base2) }.inner.base), policy)
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.inner.inner.inner.scale()
    }

    #[inline]
    fn as_image(&self) -> &dyn Image {
        self
    }
    #[inline]
    fn as_image_mut(&mut self) -> &mut dyn Image {
        self
    }
    #[inline]
    fn into_image(self: Box<Self>) -> Box<dyn Image> {
        self
    }
}

impl<T: ImageInner> AMember<AControl<AImage<T>>> {
    #[inline]
    pub fn with_content(content: image::DynamicImage) -> Box<dyn Image> {
        T::with_content(content)
    }
}
