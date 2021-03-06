use super::development::{ApplicationInnerWrapper, ApplicationInner};
use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;

static mut READY: bool = false;

thread_local! {
    pub static APPLICATION: RefCell<usize> = RefCell::new(0);
}

pub fn get<T: ApplicationInner>() -> (Option<Rc<UnsafeCell<ApplicationInnerWrapper<T>>>>, bool) {
    let ready = unsafe { READY };
    if ready {
        APPLICATION.with(|a| {
            let a = *a.borrow() as *const UnsafeCell<ApplicationInnerWrapper<T>>;
            if a.is_null() {
                (None, ready)
            } else {
                // get currently saved rointer
                let r = unsafe { Rc::from_raw(a) };
                // clone the pointer
                let ret = Some(r.clone());
                // forget the currently saved pointer, so it won't be dropped
                Rc::into_raw(r);
                (ret, ready)
            }
        })
    } else {
        (None, ready)
    }
}

pub fn init<T: ApplicationInner>(app: Rc<UnsafeCell<ApplicationInnerWrapper<T>>>) {
    if unsafe { READY } {
        panic!("Trying to access Application from a non-UI thread!") // TODO perhaps allow this if windows run in an own thread each
    } else {
        // TODO here may come the race!
        APPLICATION.with(|a| {
            *a.borrow_mut() = Rc::into_raw(app.clone()) as *const _ as usize;
        });
        unsafe {
            READY = true;
        }
    }
}
pub fn deinit<T: ApplicationInner>(_: &Rc<UnsafeCell<ApplicationInnerWrapper<T>>>) {
    if unsafe { READY } {
        // TODO here may come the race!
        APPLICATION.with(|a| {
            {
                let a = *a.borrow() as *const UnsafeCell<ApplicationInnerWrapper<T>>;
                if a.is_null() {
                    unreachable!()
                } else {
                    let r = unsafe { Rc::from_raw(a) };
                    println!("run {}", Rc::strong_count(&r));
                }
            }
            *a.borrow_mut() = 0;
        });
        unsafe {
            READY = false;
        }
    }
}
