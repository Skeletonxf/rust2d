extern crate libc;

use libc::uint32_t;
use libc::size_t;

use std;

/*
 * A struct that can be mirrored in C to facilitate returning arrays to C
 * that can then be indexed freely via Lua as cdata.
 */
#[repr(C)]
pub struct Array {
    data: *mut uint32_t,
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

pub fn array_from_vector(mut data: Vec<u32>) -> Array {
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
    array_from_vector(vec![1,4,3,8])
}
