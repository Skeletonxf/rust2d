extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

/*
 * Functions for handling FFI and Strings on the Rust side.
 */

/**
 * Creates a Rust String from a C string pointer (ie originating from Lua)
 *
 * # Safety
 *
 * The string pointer must be valid.
 */
pub unsafe fn to_rust_string(c_string_pointer: *const c_char) -> String {
    assert!(!c_string_pointer.is_null());

    CStr::from_ptr(c_string_pointer)
        .to_string_lossy()
        .into_owned()
}

/**
 * Creates a C string pointer from a Rust String and transfers ownership
 * so that the string can live long enough to be converted to a lua string
 * before Rust deallocates it and causes the pointer to be pointing
 * at nothing. The pointer must be retaken in Rust to free the memory afterwards.
 */
pub fn to_c_owned_string(rust_string: String) -> *mut c_char {
    let cstring = CString::new(rust_string).unwrap(); // todo avoid unwrap
    cstring.into_raw()
}

/*
 * Retakes the pointer created by to_c_owned_string
 * to free the memory
 */
#[no_mangle]
pub unsafe extern "C" fn free_c_owned_string(pointer: *mut c_char) {
    if pointer.is_null() {
        eprintln!("Expected to recieve non null pointer to free");
        return;
    }
    // retake pointer to free memory
    let _ = CString::from_raw(pointer);
}
