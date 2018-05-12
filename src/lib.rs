//extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;


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

#[no_mangle]
pub extern fn print_and_return(cstring: *const c_char) -> *const c_char {
    unsafe {
        let slice = CStr::from_ptr(cstring);
        let rust_str = slice.to_str().unwrap();
        println!("string returned: {}", rust_str);
        let mut rust_string = String::from(rust_str);
        rust_string.push('💖');
        println!("string now: {}", rust_string);
        return (&rust_string).as_ptr() as *const c_char
    }
}

//
// #[no_mangle]
// pub extern fn rust_multiply(size: libc::size_t, array_pointer: *const libc::int16_t)
// -> libc::int16_t {
//     println!("Got to here");
//     internal_rust_multiply(unsafe {
//         std::slice::from_raw_parts(array_pointer as *const i16, size as usize)
//     }) as libc::int16_t
// }
//
// fn internal_rust_multiply(array: &[i16]) -> i16 {
//     println!("Got to safe rust");
//     assert!(!array.is_empty());
//     println!("The array is {:?}", array); // causes seg fault
//     println!("Now leaving safe rust");
//     array[0]
// }
//
//
// #[no_mangle]
// pub extern fn unit_vector(array_pointer: *)
//
// #[no_mangle]
// pub extern fn unit_vector(mut vector: Vec<f32>) -> bool {
//     let length = vector.len() as f32;
//     println!("The vector before '{:?}'", vector);
//     for x in vector.iter_mut() {
//         *x /= length;
//     }
//     println!("The vector after '{:?}'", vector);
//     return true
// }
