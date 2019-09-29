use std::ffi::c_void;
use std::marker::PhantomData;

#[link(name = "ffbuffer")]
extern "C" {
    fn ffubuffer_build(size: u64) -> *mut c_void;
    fn ffubuffer_destroy(buffer: *mut c_void);
    fn ffubuffer_push(buffer: *mut c_void, element: *mut c_void) -> bool;
    fn ffubuffer_pop(buffer: *mut c_void) -> *mut c_void;
}

pub struct FFUnbaundedBuffer<T> {
    c_ref: *mut c_void,
    phantom: PhantomData<T>,
}
unsafe impl<T> Send for FFUnbaundedBuffer<T> {}
unsafe impl<T> Sync for FFUnbaundedBuffer<T> {}

impl<T> Drop for FFUnbaundedBuffer<T> {
    fn drop(&mut self) {
        unsafe { ffubuffer_destroy(self.c_ref) };
    }
}

impl<T> FFUnbaundedBuffer<T> {
    pub fn new(size: u64) -> FFUnbaundedBuffer<T> {
        let c_queue = unsafe { ffubuffer_build(size) };
        if c_queue.is_null() {
            panic!("Cannot create FFUnbaundedBuffer!");
        }
        FFUnbaundedBuffer {
            c_ref: c_queue,
            phantom: PhantomData,
        }
    }
    pub fn push(&self, el: Box<T>) -> Option<&str> {
        let el_ptr = Box::into_raw(el);
        let res = unsafe {
            /* Instable for now
             * let el_void = el_ptr.cast::<*mut i32>();
             */
            let el_void = std::mem::transmute::<*mut T, *mut c_void>(el_ptr);
            ffubuffer_push(self.c_ref, el_void)
        };
        if res {
            None
        } else {
            Some("error")
        }
    }
    pub fn pop(&self) -> Option<Box<T>> {
        unsafe {
            let el_void = ffubuffer_pop(self.c_ref);
            if el_void.is_null() {
                return None;
            }
            /* Instable for now
            / let el_ptr = el_void.cast::<*mut T>();
            */
            let el_ptr = std::mem::transmute::<*mut c_void, *mut T>(el_void);
            Some(Box::from_raw(el_ptr))
        }
    }
}
