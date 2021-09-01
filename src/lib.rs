#![allow(clippy::missing_safety_doc)]

extern crate libc;

use std::fmt;

use std::ffi::CStr;
use std::os::raw::c_char;

use strings::to_c_owned_string;
use strings::to_rust_string;

pub mod arrays;

pub mod tables;

pub mod strings;

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
pub unsafe extern fn print(cstring: *const c_char) {
    let slice = CStr::from_ptr(cstring);
    match slice.to_str() {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    }
}

#[no_mangle]
pub unsafe extern fn print_and_return(c_string_pointer: *const c_char) -> *mut c_char {
    let mut rust_string = to_rust_string(c_string_pointer);
    // modify the rust String
    println!("Recieved {}", rust_string);
    rust_string.push('💖');
    println!("Created {}", rust_string);
    to_c_owned_string(rust_string)
}

#[no_mangle]
pub extern fn add_two_numbers(x: u32, y: u32) -> u32 {
    x + y
}

/*
 * A struct that can be passed between C and Rust
 */
#[repr(C)]
pub struct Vector2 {
    x: u32,
    y: u32,
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
