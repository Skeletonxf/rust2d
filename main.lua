local ffi = require 'ffi'

ffi.cdef[[
void hello();
bool is_odd(unsigned int number);
char * print_and_return(const char *string);
void free_c_owned_string(char *string);
void print(const char *string);
uint32_t add_two_numbers(uint32_t, uint32_t);
void print_array(const uint32_t *array, size_t length);
// mirror the rust Vector2 struct definition
typedef struct vector2 {
  uint32_t x;
  uint32_t y;
} vector2_t;
vector2_t vector2_swap(vector2_t);
]]

loverust = ffi.load('./target/release/libloverust.so')

local arrays = require 'src.arrays'

loverust.hello()
print('Is 1 odd? ' .. tostring(loverust.is_odd(1)))

local cstring = loverust.print_and_return("💖plswork")
local luaSting = ffi.string(cstring)
print('It worked! ' .. luaSting)
loverust.free_c_owned_string(cstring)
print('And no memory leak!')

print('We still have the lua string from Rust')
loverust.print(luaSting)

print('Addition ' .. loverust.add_two_numbers(1, 2))

local carray = ffi.new("int[3]", {1,2,3})
print('Rust inspection of c array:')
loverust.print_array(carray, 3)

local vector2 = ffi.new("struct vector2", {1, 2})
local swapped = loverust.vector2_swap(vector2)

print((ffi.new("int[3]", {1,4,9}))[0])

local array = arrays.new(loverust.generate_array())
print(array)
array:free()
array = nil
