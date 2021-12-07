use std::os::raw::{c_char, c_int, c_ulong};

pub enum Display {}

#[link(name = "X11")]
extern "C" {
    pub fn XOpenDisplay(display: *const c_char) -> *mut Display;
    pub fn XDefaultRootWindow(dpy: *mut Display) -> c_ulong;
    pub fn XStoreName(d: *mut Display, w: c_ulong, s: *const c_char) -> c_int;
    pub fn XFlush(d: *mut Display) -> c_int;
    pub fn XCloseDisplay(dpy: *mut Display) -> c_int;
}
