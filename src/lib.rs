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

/*
 * Creates a Rust String from a C string pointer
 */
fn to_rust_string(c_string_pointer: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(c_string_pointer).to_string_lossy().into_owned()
    }
}

/*
 * Creates a C string pointer from a Rust String
 */
fn to_c_string(rust_string: &str) -> *const c_char {
    let cstring = CString::new(rust_string).unwrap(); // todo avoid unwrap
    cstring.as_ptr()
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

//
// unsafe {
//     let slice = CStr::from_ptr(c_string_pointer);
//     // convert it to a rust &str
//     let rust_str = slice.to_str().unwrap(); // todo avoid unwrapping
//     println!("string returned: {}", rust_str);
//     // create a rust String
//     let mut rust_string = String::from(rust_str);
//     // modify the rust String
//     rust_string.push('💖');
//     println!("string now: {}", rust_string);
//     // we need to return a CString pointer and not a rust String
//     let cstring = CString::new(rust_string).unwrap(); // todo avoid unwrapping
//     return (&cstring).as_ptr()
// }

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
