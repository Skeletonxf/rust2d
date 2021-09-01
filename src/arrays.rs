extern crate libc;

use libc::size_t;

// Same precision as Lua number (double)
// also the same as libc::c_double;
type LuaNumber = f64;

use std;
use std::slice;

/**
 * A struct that can be mirrored in C to facilitate returning arrays to C
 * that can then be indexed freely via Lua as cdata.
 */
#[repr(C)]
pub struct Array {
    data: *mut LuaNumber,
    length: size_t,
}

#[no_mangle]
pub extern fn free_array(array: Array) {
    assert!(!array.data.is_null());
    unsafe {
        // reclaim the Vec so it can be dropped
        Box::from_raw(array.data);
    }
}

/**
 * Converts a Rust Vec to an Array to pass through FFI
 */
pub fn vec_to_array(mut data: Vec<LuaNumber>) -> Array {
    let array = Array {
        data: data.as_mut_ptr(),
        length: data.len(),
    };
    // forget about the Vec so it is not dropped
    // at the end of this function
    std::mem::forget(data);
    array
}

#[no_mangle]
pub extern fn generate_array() -> Array {
    vec_to_array(vec![1.0f64, 4.0f64, 3.0f64, 8.0f64])
}

/**
 * Prints an Array originating from LuaJIT
 *
 * # Safety
 *
 * The array pointer must be valid.
 */
#[no_mangle]
pub unsafe extern fn print_array(c_array_pointer: *const u32, length: size_t) {
    let array_slice = {
        assert!(!c_array_pointer.is_null());

        slice::from_raw_parts(c_array_pointer, length as usize)
    };
    println!("{:?}", array_slice);
}
