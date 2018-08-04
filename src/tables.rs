extern crate libc;

use std::collections::HashMap;
use std::os::raw::c_char;

use strings;

// Same precision as Lua number (double)
type LuaNumber = f64;

#[derive(Debug, Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(LuaNumber),
    String(String),
    Table(Table),
    // Userdata, Functions and Threads are not intended to be
    // passed through FFI
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LuaKey {
    String(String),
    // Lua can hash other values but it is best to stick to Strings
}

// Rust representation of a Lua table
#[derive(Debug, Clone)]
pub struct Table {
    array: Vec<LuaValue>,
    hash_map: HashMap<LuaKey, LuaValue>,
}

impl PartialEq for Table {
    fn eq(&self, other: &Table) -> bool {
        // Lua tables are compared by pointer equality
        // and we cannot compare by contents here because f64
        // does not implement Eq so we do the same.
        self as *const _ == other as *const _
    }
}
impl Eq for Table {}

impl Table {
    fn new() -> Table {
        Table {
            array: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    fn add_value(&mut self, value: LuaValue) {
        self.array.push(value);
    }

    fn put_key_value(&mut self, key: LuaKey, value: LuaValue) {
        self.hash_map.insert(key, value);
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

#[no_mangle]
pub extern fn tables_add_number(
    pointer: *mut Table,
    value: LuaNumber,
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    table.add_value(LuaValue::Number(value));
}

#[no_mangle]
pub extern fn tables_add_string(
    pointer: *mut Table,
    c_string_pointer_value: *const c_char,
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    let value = strings::to_rust_string(c_string_pointer_value);
    table.add_value(LuaValue::String(value));
}

#[no_mangle]
pub extern fn tables_add_boolean(
    pointer: *mut Table,
    value: bool,
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    table.add_value(LuaValue::Boolean(value));
}

#[no_mangle]
pub extern fn tables_add_nil(pointer: *mut Table) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    table.add_value(LuaValue::Nil);
}

#[no_mangle]
pub extern fn tables_add_table(
    pointer: *mut Table,
    table_pointer_value: *mut Table,
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    if table_pointer_value.is_null() {
        eprintln!("Expected pointer to Table value to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    // Take back ownership of the Table to move into the main Table
    // This means the Lua side does not need to free subtables
    // when constructing a Table
    let value = unbox(
        unsafe {
            Box::from_raw(table_pointer_value)
        }
    );
    table.add_value(LuaValue::Table(value));
}

#[no_mangle]
pub extern fn tables_put_string_string(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    c_string_pointer_value: *const c_char,
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    let value = strings::to_rust_string(c_string_pointer_value);
    table.put_key_value(LuaKey::String(key), LuaValue::String(value));
}

#[no_mangle]
pub extern fn tables_put_string_boolean(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: bool
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    table.put_key_value(LuaKey::String(key), LuaValue::Boolean(value));
}

#[no_mangle]
pub extern fn tables_put_string_number(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: LuaNumber
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    table.put_key_value(LuaKey::String(key), LuaValue::Number(value));
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[no_mangle]
pub extern fn tables_put_string_table(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    table_pointer_value: *mut Table
) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    if table_pointer_value.is_null() {
        eprintln!("Expected pointer to Table value to not be null");
        return;
    }
    let table = unsafe {
        &mut *pointer
    };
    let key = strings::to_rust_string(c_string_pointer_key);
    // Take back ownership of the Table to move into the main Table
    // This means the Lua side does not need to free subtables
    // when constructing a Table
    let value = unbox(
        unsafe {
            Box::from_raw(table_pointer_value)
        }
    );
    table.put_key_value(LuaKey::String(key), LuaValue::Table(value));
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
