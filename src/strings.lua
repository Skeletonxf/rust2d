local strings = {
  _DESCRIPTION = [[
Module for manipulating strings over FFI
  ]],
  _URL = 'https://github.com/Skeletonxf/rust2d',
  _LICENSE = 'MPL2'
}

local ffi = require 'ffi'
local loverust = require 'src.loverust'

ffi.cdef[[
void free_c_owned_string(char *string);
]]

function strings.copy(c_string)
  return ffi.string(c_string)
end

function strings.free(c_string)
  loverust.free_c_owned_string(c_string)
end

-- Copies a C string to a Lua string, then frees
-- the C string, leaving the Lua copy intact
function strings.take(c_string)
  local luaString = strings.copy(c_string)
  strings.free(c_string)
  return luaString
end

return strings
