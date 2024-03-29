extern crate libc;

use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_char;

use strings;

/** Same precision as Lua number (double) */
type LuaNumber = f64;

/** Enumeration of lua table value types.
 *
 * Cannot hash because f64 is not hashable.
 */
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

impl fmt::Display for LuaValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuaValue::Nil => write!(f, "nil"),
            LuaValue::Boolean(boolean) => write!(f, "{}", boolean),
            LuaValue::Number(number) => write!(f, "{}", number),
            LuaValue::String(string) => write!(f, "{}", string),
            LuaValue::Table(table) => write!(f, "{}", table),
        }
    }
}

/** Enumeration of lua table key types. */
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LuaKey {
    String(String),
    // Lua can hash other values but it is best to stick to Strings
}

impl fmt::Display for LuaKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuaKey::String(s) => write!(f, "{}", s),
        }
    }
}

/// Rust representation of a Lua table
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
        std::ptr::eq(self, other)
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

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?; // write literal {
        for (index, value) in self.array.iter().enumerate() {
            write!(f, "{}", value)?;
            if index < (self.array.len() - 1) {
                write!(f, ", ")?;
            }
        }
        if !self.array.is_empty() && !self.hash_map.is_empty() {
            write!(f, ", ")?;
        }
        for (index, (key, value)) in self.hash_map.iter().enumerate() {
            write!(f, "{}: {}", key, value)?;
            if index < (self.hash_map.len() - 1) {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}") // write literal }
    }
}

/**
 * Gets a reference to the Table from a pointer (if it exists)
 *
 * # Safety
 *
 * The table pointer must be valid.
 */
unsafe fn get_table<'a>(pointer: *mut Table) -> Option<&'a mut Table> {
    if pointer.is_null() {
        return None;
    }
    Some(&mut *pointer)
}

// argument is boxed because it was heap allocated to give to lua and now we're unboxing it
#[allow(clippy::boxed_local)]
fn unbox<T>(value: Box<T>) -> T {
    *value
}

#[no_mangle]
pub extern "C" fn tables_new_empty_table() -> *mut Table {
    Box::into_raw(Box::new(Table::new()))
}

#[no_mangle]
pub unsafe extern "C" fn tables_debug(pointer: *mut Table) {
    match get_table(pointer) {
        Some(table) => println!("{:?}\n{}", table, table),
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_add_number(pointer: *mut Table, value: LuaNumber) {
    match get_table(pointer) {
        Some(table) => table.add_value(LuaValue::Number(value)),
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_add_string(
    pointer: *mut Table,
    c_string_pointer_value: *const c_char,
) {
    let value = strings::to_rust_string(c_string_pointer_value);
    match get_table(pointer) {
        Some(table) => table.add_value(LuaValue::String(value)),
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_add_boolean(pointer: *mut Table, value: bool) {
    match get_table(pointer) {
        Some(table) => table.add_value(LuaValue::Boolean(value)),
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_add_nil(pointer: *mut Table) {
    match get_table(pointer) {
        Some(table) => table.add_value(LuaValue::Nil),
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_add_table(pointer: *mut Table, table_pointer_value: *mut Table) {
    match get_table(pointer) {
        Some(table) => {
            if table_pointer_value.is_null() {
                eprintln!("Expected pointer to Table value to not be null");
                return;
            }
            // Take back ownership of the Table to move into the main Table
            // This means the Lua side does not need to free subtables
            // when constructing a Table
            let value = unbox(Box::from_raw(table_pointer_value));
            table.add_value(LuaValue::Table(value));
        }
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_put_string_string(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    c_string_pointer_value: *const c_char,
) {
    match get_table(pointer) {
        Some(table) => {
            let key = strings::to_rust_string(c_string_pointer_key);
            let value = strings::to_rust_string(c_string_pointer_value);
            table.put_key_value(LuaKey::String(key), LuaValue::String(value));
        }
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_put_string_boolean(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: bool,
) {
    match get_table(pointer) {
        Some(table) => {
            let key = strings::to_rust_string(c_string_pointer_key);
            table.put_key_value(LuaKey::String(key), LuaValue::Boolean(value));
        }
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_put_string_number(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    value: LuaNumber,
) {
    match get_table(pointer) {
        Some(table) => {
            let key = strings::to_rust_string(c_string_pointer_key);
            table.put_key_value(LuaKey::String(key), LuaValue::Number(value));
        }
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_put_string_table(
    pointer: *mut Table,
    c_string_pointer_key: *const c_char,
    table_pointer_value: *mut Table,
) {
    match get_table(pointer) {
        Some(table) => {
            if table_pointer_value.is_null() {
                eprintln!("Expected pointer to Table value to not be null");
                return;
            }
            let key = strings::to_rust_string(c_string_pointer_key);
            // Take back ownership of the Table to move into the main Table
            // This means the Lua side does not need to free subtables
            // when constructing a Table
            let value = unbox(Box::from_raw(table_pointer_value));
            table.put_key_value(LuaKey::String(key), LuaValue::Table(value));
        }
        None => eprintln!("Expected pointer to Table to not be null"),
    };
}

#[no_mangle]
pub unsafe extern "C" fn tables_free_table(pointer: *mut Table) {
    if pointer.is_null() {
        eprintln!("Expected pointer to Table to not be null");
        return;
    }
    // take back to allow it to be freed when exiting this block
    Box::from_raw(pointer);
}
