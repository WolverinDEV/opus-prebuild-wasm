/* define for lib opus */
use std::os::raw::{c_char, c_int};
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn __opus_fprintf(fmt: *const c_char /* , mut ap: ... */, _: i32) -> c_int {
    println!("opus fprintf({}, ...);", unsafe { CStr::from_ptr(fmt) }.to_str().unwrap_or("invalid utf-8"));
    0
}

#[no_mangle]
pub unsafe extern "C" fn opus_alloc(size: u32) -> *mut u8 {
    let mut vec = std::mem::ManuallyDrop::new( Vec::with_capacity((size + 4) as usize));
    let mut ptr = vec.as_mut_ptr() as *mut u32;

    ptr.write(size);
    ptr.offset(1) as *mut u8
}

#[no_mangle]
pub unsafe extern "C" fn opus_free(pointer: *mut u32) -> i32 {
    let mut pointer = pointer.sub(1);
    Vec::from_raw_parts(pointer, 0, pointer.read() as usize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __opus_abort(_: i32) {
    panic!("opus abort()");
}

#[no_mangle]
pub unsafe extern "C" fn __opus_abs(value: c_int) -> c_int {
    if value > 0 { value } else { -value }
}