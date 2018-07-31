local tables = {
  _DESCRIPTION = [[
A utility to convert somewhat arbitrary lua tables into a format that
can be manipulated in Rust and sent back.
  ]],
  _URL = 'https://github.com/Skeletonxf/rust2d',
  _LICENSE = 'MPL2'
}

local ffi = require 'ffi'
local loverust = require 'src.loverust'

-- Opaque object
-- http://jakegoulding.com/rust-ffi-omnibus/objects/
ffi.cdef[[
typedef struct table_S table_t;
table_t * tables_new_empty_table();
void tables_import_array(table_t *, const double *array, size_t length);
array_t tables_export_array(table_t *);
]]

local arrays = require 'src.arrays'

local Table = {}
Table.__index = Table
local table = {}

local function export(lua_table, rust_table)
  local array = {}
  -- loop over all pairs in the table and
  -- assign all numerical keys to the array
  for k, v in pairs(lua_table) do
    if type(k) == 'number' then
      array[k] = v
    end
    -- TODO: make assignment to hash part of rust table
  end
  if #array > 0 then
    -- process array portion of table
    rust_table:setArray(array)
    local result = rust_table:getArray()
    print(result)
    result:free()
  end
end

function tables.export(lua_table)
  return export(lua_table, table.new())
end

-- TODO: tables.import

function table.new()
  local o = {}
  o.table = loverust.tables_new_empty_table()
  setmetatable(o, Table)
  return o
end

-- Takes a lua array and creates a c array from it
-- to set on the Rust Table.
function Table.setArray(self, array)
  loverust.tables_import_array(
      self.table,
      ffi.new("double[" .. tostring(#array) .. "]", array),
      #array)
end

-- Gets a copy of the Rust Table's array.
-- This will need freeing.
function Table.getArray(self)
  return arrays.new(loverust.tables_export_array(self.table))
end

return tables
