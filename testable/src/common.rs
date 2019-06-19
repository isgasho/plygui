pub use plygui_api::development::*;
pub use plygui_api::{callbacks, controls, defaults, ids, layout, types, utils};
pub use plygui_api::external::image;

pub use std::borrow::Cow;
pub use std::ffi::{CString, IntoStringError, OsStr};
pub use std::marker::PhantomData;
pub use std::os::windows::ffi::OsStrExt;
pub use std::{cmp, mem, ops, ptr, str, sync::mpsc};
pub use std::rc::Rc;
pub use std::cell::RefCell;

pub const DEFAULT_PADDING: i32 = 2;

pub type InnerId = *mut MemberBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestableId(InnerId);

impl From<InnerId> for TestableId {
    #[inline]
    fn from(a: InnerId) -> TestableId {
        TestableId(a)
    }
}
impl From<TestableId> for InnerId {
    #[inline]
    fn from(a: TestableId) -> InnerId {
        a.0
    }
}
impl From<TestableId> for usize {
    #[inline]
    fn from(a: TestableId) -> usize {
        a.0 as usize
    }
}
impl NativeId for TestableId {}

#[repr(C)]
pub struct TestableControlBase<T: controls::Control + Sized> {
    pub id: InnerId,
    parent: Option<InnerId>,
    visibility: types::Visibility,
    _marker: PhantomData<T>,
}

impl<T: controls::Control + Sized> TestableControlBase<T> {
    pub fn new() -> TestableControlBase<T> {
        TestableControlBase {
            id: ptr::null_mut(),
            parent: None,
            visibility: types::Visibility::Visible,
            _marker: PhantomData,
        }
    }

    pub fn parent_id(&self) -> Option<InnerId> {
        self.parent
    }
    pub fn parent(&self) -> Option<&MemberBase> {
    	self.parent.map(|p| &*(p as *const MemberBase))
    }
    pub fn parent_mut(&mut self) -> Option<&mut MemberBase> {
       self.parent.map(|p| &mut *(p as *mut MemberBase))
    }
    /*pub fn root(&self) -> Option<&MemberBase> {
    	let mut p = self.parent().map(|p| p.as_member());
    	while let Some(pp) = p {
    		let c = pp.is_control();
    		if c.is_none() {
    			return Some(pp.base());
    		} else {
    			pp = c.unwrap().parent();
    		}
    	}
    	None
    }
    pub fn root_mut(&mut self) -> Option<&mut MemberBase> {
        unsafe {
            let parent_id = winuser::GetAncestor(self.id, 2); //GA_ROOT
            if parent_id == self.id {
                return None;
            }

            let parent_ptr = winuser::GetWindowLongPtrW(parent_id, winuser::GWLP_USERDATA);
            mem::transmute(parent_ptr as *mut c_void)
        }
    }*/
    pub fn as_outer(&self) -> &T {
        member_from_id::<T>(self.id.into()).unwrap()
    }
    pub fn as_outer_mut(&self) -> &mut T {
        member_from_id::<T>(self.id.into()).unwrap()
    }
    pub fn invalidate(&mut self) {
        if self.id.is_null() {
            return;
        }
        let parent_id = self.parent_id();
        let this = self.as_outer_mut();
        if this.is_skip_draw() {
            return;
        }
        if let Some(parent_id) = parent_id {
            if let Some(mparent) = member_base_from_id(parent_id.into()) {
                let (pw, ph) = mparent.as_member().is_has_size().unwrap().size();
                let (_, _, changed) = this.measure(pw, ph);

                if let Some(cparent) = mparent.as_member_mut().is_control_mut() {
                    if changed && !cparent.is_skip_draw() {
                        cparent.invalidate();
                    }
                } else {
                    this.draw(None);
                }
            }
        }
    }
    pub fn on_set_visibility(&mut self, visibility: types::Visibility) -> bool {
        self.visibility = visibility;
        true
    }
}

#[inline]
pub(crate) unsafe fn cast_id<'a, T>(id: InnerId) -> Option<&'a mut T>
where
    T: Sized,
{
    if id.is_null() {
        None
    } else {
        Some(mem::transmute(&mut *id))
    }
}
#[inline]
pub fn member_from_id<'a, T>(id: TestableId) -> Option<&'a mut T>
where
    T: Sized + controls::Member,
{
    unsafe { cast_id(id.into()) }
}
#[inline]
pub fn member_base_from_id<'a>(id: TestableId) -> Option<&'a mut MemberBase> {
    unsafe { cast_id(id.into()) }
}

/*pub unsafe fn make_menu(menu: windef::HMENU, mut items: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>) {
    let mut options = Vec::new();
    let mut help = Vec::new();

    let append_item = |menu, label, action, storage: &mut Vec<callbacks::Action>| {
        let wlabel = str_to_wchar(label);
        let id = storage.len();
        storage.push(action);
        winuser::AppendMenuW(menu, winuser::MF_STRING, id, wlabel.as_ptr());
    };
    let append_level = |menu, label, items, storage: &mut Vec<callbacks::Action>| {
        let wlabel = str_to_wchar(label);
        let submenu = winuser::CreateMenu();
        make_menu(submenu, items, storage);
        winuser::AppendMenuW(menu, winuser::MF_POPUP, submenu as usize, wlabel.as_ptr());
    };
    let make_special = |menu, mut special: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>| {
        for item in special.drain(..) {
            match item {
                types::MenuItem::Action(label, action, _) => {
                    append_item(menu, label, action, storage);
                }
                types::MenuItem::Sub(label, items, _) => {
                    append_level(menu, label, items, storage);
                }
                types::MenuItem::Delimiter => {
                    winuser::AppendMenuW(menu, winuser::MF_SEPARATOR, 0, ptr::null_mut());
                }
            }
        }
    };

    for item in items.drain(..) {
        match item {
            types::MenuItem::Action(label, action, role) => match role {
                types::MenuItemRole::None => {
                    append_item(menu, label, action, storage);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Action(label, action, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Action(label, action, role));
                }
            },
            types::MenuItem::Sub(label, items, role) => match role {
                types::MenuItemRole::None => {
                    append_level(menu, label, items, storage);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Sub(label, items, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Sub(label, items, role));
                }
            },
            types::MenuItem::Delimiter => {
                winuser::AppendMenuW(menu, winuser::MF_SEPARATOR, 0, ptr::null_mut());
            }
        }
    }

    make_special(menu, options, storage);
    make_special(menu, help, storage);
}*/
