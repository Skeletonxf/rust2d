local ffi = require 'ffi'

ffi.cdef[[
void hello();
bool is_odd(unsigned int number);
char * print_and_return(const char *string);
void print(const char *string);
uint32_t add_two_numbers(uint32_t, uint32_t);
// mirror the rust Vector2 struct definition
typedef struct vector2 {
  uint32_t x;
  uint32_t y;
} vector2_t;
vector2_t vector2_swap(vector2_t);
]]

local loverust = require 'src.loverust'

local arrays = require 'src.arrays'
local strings = require 'src.strings'

loverust.hello()
print('Is 1 odd? ' .. tostring(loverust.is_odd(1)))

local cstring = loverust.print_and_return("💖plswork")
local luaSting = strings.copy(cstring)
print('It worked! ' .. luaSting)
strings.free(cstring)
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

local complicatedTable = {
  1,2,4,
  baz = 4,
  barbaz = true,
  foo = 'bar',
  foobar = {
    1, 6, 'baz', foobaz = {
      4, 'bazfoo'
    }
  }
}

local tables = require 'src.tables'
tables.export(complicatedTable)
