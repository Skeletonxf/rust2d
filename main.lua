ffi = require 'ffi'

-- https://michael-f-bryan.github.io/cheat-sheets/Rust/rust_interop.html
-- declare the external rust/c functions to use
-- https://en.wikipedia.org/wiki/C_data_types
-- https://doc.rust-lang.org/book/first-edition/ffi.html
-- https://doc.rust-lang.org/book/second-edition/ch03-02-data-types.html
-- https://stackoverflow.com/questions/49591678/is-it-possible-to-pass-arrays-from-rust-to-c
-- http://wiki.luajit.org/FFI-Parameterized-Types
-- https://github.com/neomantra/lds/tree/master/tests
-- http://luajit.org/ext_ffi_semantics.html#clang
-- https://stackoverflow.com/questions/29182843/pass-a-c-array-to-a-rust-function#29183118

--struct array {
--  unsigned float * data;
--  size_t size;
--}
-- float[] unit_vector(float v[]);
-- int rust_multiply(int a[]);
ffi.cdef[[
void hello();
bool is_odd(unsigned int number);
const char * print_and_return(const char *string);
void print(const char *string);
]]

loverust = ffi.load('./target/release/libloverust.so')

loverust.hello()
print('Is 1 odd? ' .. tostring(loverust.is_odd(1)))
--
-- local array = ffi.new("int[2]", {1,2})
-- print('Array going to rust ' .. tostring(array))
--
-- print('Unit vector ' .. tostring(loverust.rust_multiply(array)))
local cstring = loverust.print_and_return("💖plswork")
local luaSting = ffi.string(cstring)
print('It worked! ' .. luaSting)

loverust.print(luaSting)
