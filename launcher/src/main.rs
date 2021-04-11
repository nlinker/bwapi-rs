use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::cell::{RefCell, UnsafeCell};
use std::{mem, ptr};
use std::mem::MaybeUninit;

static mut GLOBAL_RAW: Option<String> = None;
static GLOBAL: String = String::new();


fn main() {
    let v1 = "Hello, world".to_string();
    println!("Original address: {:p}", &v1);
    let mut v2;
    // Override rust protections on reading from uninitialized memory
    unsafe { v2 = MaybeUninit::uninit().assume_init(); }
    let addr = &mut v2 as *mut _;
    // ptr::write() though it takes v1 by value, v1s destructor is not run when it goes out of
    // scope, which is good since then we'd have a vector of free'd vectors
    unsafe { ptr::write(addr, v1); }
    println!("New address: {:p}", &v2);
    println!("New value: {}", v2);
}
