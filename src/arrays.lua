local arrays = {
  _DESCRIPTION = [[
A wrapper around a struct containing an array originating from Rust.
Luajit gives us a reference to cdata arrays so we can index the values.
The struct contains a length parameter so we know how many elements
are in the array.

This module assumes `loverust` is a global reference to the dynamic lib.
  ]],
  _URL = 'https://github.com/Skeletonxf/rust2d',
  _LICENSE = 'MPL2'
}

local ffi = require 'ffi'

ffi.cdef[[
// wrapper around an array for recieving from rust functions
typedef struct array {
  uint32_t * data;
  size_t length;
} array_t;
void free_array(array_t);
array_t generate_array();
]]


local Array = {}
Array.__index = Array

-- wraps a struct array held in C
function arrays.new(structArray)
  local array = {}
  array.struct = structArray
  setmetatable(array, Array)
  return array
end

function Array.length(self)
  if not self.struct then
    error('No struct to query', 2, debug.traceback())
  end
  return tonumber(self.struct.length)
end

function Array.get(self, i)
  if not self.struct then
    error('No struct to query', 2, debug.traceback())
  end
  local data = self.struct.data
  if i >= self:length() or i < 0 then
    error('Out of bounds for array', 2, debug.traceback())
  end
  return self.struct.data[i]
end

function Array.free(self)
  -- call Rust to free the vector held in the struct
  -- and then nil the fields in this wrapper
  loverust.free_array(self.struct)
  array = {}
end

function Array.__tostring(self)
  local s = '['
  local i = 0
  while i < (self:length()) do
    s = s .. tostring(self:get(i))
    if i < self:length() - 1 then
      s = s .. ','
    end
    i = i + 1
  end
  s = s .. ']'
  return s
end

return arrays
