extern crate libc;

use arrays;
use arrays::Array;

use libc::size_t;

use std::collections::HashMap;
use std::slice;
use std::os::raw::c_char;

use strings;

// Same precision as Lua number (double)
type LuaNumber = f64;

#[derive(Debug)]
pub enum LuaType {
    Nil, // Lua cannot have nil values but can have a nil key
    Boolean(bool),
    Number(LuaNumber),
    String(String), // Lua can have non string keys
    Table(Table),
    // Userdata, Functions and Threads are not intended to be
    // passed through FFI
}

// Rust representation of a Lua table
#[derive(Debug)]
pub struct Table {
    array: Vec<LuaNumber>, // TODO: Lua can have nil numbers
    hash_map: HashMap<String, LuaType>, // TODO
}

impl Table {
    fn new() -> Table {
        Table {
            array: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    /**
     * Returns a copy of this array as repr(C)
     * The Array should be freed using the arrays module
     */
    fn export_array(&mut self) -> Array {
        arrays::vec_to_array(self.array.clone())
    }

    fn import_array(&mut self, data: Vec<LuaNumber>) {
        self.array = data;
    }

    fn put_string_string(&mut self, key: String, value: String) {
        self.hash_map.insert(key, LuaType::String(value));
    }

    fn put_string_boolean(&mut self, key: String, value: bool) {
        self.hash_map.insert(key, LuaType::Boolean(value));
    }

    fn put_string_number(&mut self, key: String, value: LuaNumber) {
        self.hash_map.insert(key, LuaType::Number(value));
    }
}

#[no_mangle]
pub extern fn tables_new_empty_table() -> *mut Table {
    Box::into_raw(Box::new(Table::new()))
}

#[no_mangle]
pub extern fn tables_debug(pointer: *mut Table) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
    }
    let table = unsafe {
        &mut *pointer
    };
    println!("{:?}", table)
}

/**
 * Imports the C array into the Table's array field
 */
#[no_mangle]
pub extern fn tables_import_array(
    pointer: *mut Table,
    c_array_pointer: *const LuaNumber,
    length: size_t
) {
    if c_array_pointer.is_null() {
        eprintln!("Expected pointer to C array to not be null");
        return;
    }
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let array_slice = unsafe {
        slice::from_raw_parts(c_array_pointer, length as usize)
    };
    let table = unsafe {
        &mut *pointer
    };
    table.import_array(array_slice.to_vec());
}

/**
 * Exports a copy of the Table's array field as an Array
 * from the Arrays module
 */
#[no_mangle]
pub extern fn tables_export_array(pointer: *mut Table) -> Array {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        panic!();
    }
    let table = unsafe {
        &mut *pointer
    };
    table.export_array()
}

#[no_mangle]
pub extern fn tables_put_string_string(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    c_string_pointer_value: *const c_char
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        panic!();
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    let value = strings::to_rust_string(c_string_pointer_value);
    table.put_string_string(key, value);
}

#[no_mangle]
pub extern fn tables_put_string_boolean(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: bool
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        panic!();
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    table.put_string_boolean(key, value);
}

#[no_mangle]
pub extern fn tables_put_string_number(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: LuaNumber
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        panic!();
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    table.put_string_number(key, value);
}

#[no_mangle]
pub extern fn tables_free_table(pointer: *mut Table) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    unsafe {
        // take back to allow it to be freed when exiting this block
        Box::from_raw(pointer);
    }
}
