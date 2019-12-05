use crate::{Adapted, Adapter, Control, Change, HasLabel};
use crate::{imp, development};
use crate::AsAny;
use std::any::Any;

pub struct StringVecAdapter<C: HasLabel> {
    items: Vec<String>,
    on_item_change: Option<development::AdapterInnerCallback>
}

impl<C: HasLabel> StringVecAdapter<C> {
    pub fn new() -> Self {
        StringVecAdapter { items: Vec::new(), on_item_change: None }
    }
    pub fn with_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: Iterator<Item=T> {
        let mut t = Self::new();
        for item in i {
            t.items.push(String::from(item.as_ref()));
        }
        t
    }
    pub fn with_into_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: IntoIterator<Item=T> {
        Self::with_iterator(i.into_iter())
    }

    pub fn text_at(&self, i: usize) -> Option<&String> {
        self.items.get(i)    
    }
    pub fn text_at_mut(&mut self, i: usize) -> Option<&mut String> {
        self.items.get_mut(i)    
    }
    pub fn push<T: AsRef<str>>(&mut self, arg: T) {
        let i = self.items.len();
        self.items.push(String::from(arg.as_ref()));
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            cb.on_item_change(Change::Added(i))
        }
    }
    pub fn pop(&mut self) -> Option<String> {
        let t = self.items.pop();
        let i = self.items.len();
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            cb.on_item_change(Change::Removed(i))
        }
        t
    }
}
impl<C: HasLabel> AsAny for StringVecAdapter<C> {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
impl<C: HasLabel> Adapter for StringVecAdapter<C> {
    fn len(&self) -> usize {
        self.items.len()
    }
	fn spawn_item_view(&mut self, i: usize, _parent: &dyn Adapted) -> Box<dyn Control> {
	    let control = C::spawn();
	    control.set_label(self.items[i].as_str().into());
    	control
	}
}
impl<C: HasLabel> development::AdapterInner for StringVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<development::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
