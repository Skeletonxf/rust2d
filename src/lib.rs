extern crate libc;

use libc::uint32_t;
use libc::size_t;

use std::fmt;

use std::slice;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub mod arrays;

#[no_mangle]
pub extern fn hello() {
    println!("Hello from rust");
}

#[no_mangle]
pub extern fn is_odd(n: u16) -> bool {
    (n % 2) == 0
}

/**
 * Prints the c string supplied
 * Unlike lua/c code this prints unicode properly
 */
#[no_mangle]
pub extern fn print(cstring: *const c_char) {
    unsafe {
        let slice = CStr::from_ptr(cstring);
        match slice.to_str() {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    }
}

/*
 * Creates a Rust String from a C string pointer
 */
fn to_rust_string(c_string_pointer: *const c_char) -> String {
    unsafe {
        assert!(!c_string_pointer.is_null());

        CStr::from_ptr(c_string_pointer).to_string_lossy().into_owned()
    }
}

/*
 * Creates a C string pointer from a Rust String and transfers ownership
 * so that the string can live long enough to be converted to a lua string
 * before Rust deallocates it and causes the pointer to be pointing
 * at nothing. The pointer must be retaken in Rust to free the memory afterwards.
 */
fn to_c_owned_string(rust_string: String) -> *mut c_char {
    let cstring = CString::new(rust_string).unwrap(); // todo avoid unwrap
    cstring.into_raw()
}

/*
 * Retakes the pointer created by to_c_owned_string
 * to free the memory
 */
#[no_mangle]
pub extern fn free_c_owned_string(pointer: *mut c_char) {
    unsafe {
        if pointer.is_null() {
            eprintln!("Expected to recieve non null pointer to free");
            return;
        }
        // retake pointer to free memory
        let _ = CString::from_raw(pointer);
    }
}

#[no_mangle]
pub extern fn print_and_return(c_string_pointer: *const c_char) -> *mut c_char {
    let mut rust_string = to_rust_string(c_string_pointer);
    // modify the rust String
    println!("Recieved {}", rust_string);
    rust_string.push('💖');
    println!("Created {}", rust_string);
    return to_c_owned_string(rust_string)
}

#[no_mangle]
pub extern fn add_two_numbers(x: uint32_t, y: uint32_t) -> uint32_t {
    x + y
}


#[no_mangle]
pub extern fn print_array(c_array_pointer: *const uint32_t, length: size_t) {
    let array_slice = unsafe {
        assert!(!c_array_pointer.is_null());

        slice::from_raw_parts(c_array_pointer, length as usize)
    };
    println!("{:?}", array_slice);
}

/*
 * A struct that can be passed between C and Rust
 */
#[repr(C)]
pub struct Vector2 {
    x: uint32_t,
    y: uint32_t,
}

/*
 * Display implementation for easy viewing
 */
impl fmt::Display for Vector2 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("{")?;
        fmt.write_str(&self.x.to_string())?;
        fmt.write_str(", ")?;
        fmt.write_str(&self.y.to_string())?;
        fmt.write_str("}")?;
        Ok(())
    }
}

#[no_mangle]
pub extern fn vector2_swap(vector2: Vector2) -> Vector2 {
    println!("Before {}", vector2);
    let swapped = Vector2 {
        x: vector2.y,
        y: vector2.x
    };
    println!("After {}", swapped);
    swapped
}
