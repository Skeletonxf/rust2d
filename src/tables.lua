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
void tables_add_string(table_t *, const char *string);
void tables_add_number(table_t *, double);
void tables_add_nil(table_t *);
void tables_add_table(table_t *, table_t *);
void tables_put_string_string(table_t *, const char *string, const char *string);
void tables_put_string_boolean(table_t *, const char *string, bool);
void tables_put_string_number(table_t *, const char *string, double);
void tables_put_string_table(table_t *, const char *string, table_t *);
void tables_debug(table_t *);
void tables_free_table(table_t *);
]]

local arrays = require 'src.arrays'

local Table = {}
Table.__index = Table
local table = {}

local function export(lua_table, rust_table)
  -- First loop through numerical indexes
  -- in order and add to Rust Table array.
  for _, v in ipairs(lua_table) do
    -- This will turn the array into 1 indexed on the Rust side
    rust_table:add(v)
    lua_table[v] = nil
  end
  -- Now loop through other pairs and add to
  -- Rust Table hash map.
  for k, v in pairs(lua_table) do
    rust_table:put(k, v)
  end
  rust_table:debug()
  return rust_table
end

function tables.export(lua_table)
  return export(lua_table, table.new())
end

function table.new()
  local o = {}
  o.table = loverust.tables_new_empty_table()
  setmetatable(o, Table)
  return o
end

-- Adds a value to the array part of the Rust Table
function Table.add(self, value)
  if type(value) == 'number' then
    loverust.tables_add_number(self.table, value)
  end
  if type(value) == 'string' then
    loverust.tables_add_string(self.table, value)
  end
  if type(value) == 'nil' then
    loverust.tables_add_nil(self.table)
  end
  if type(value) == 'table' then
    -- subtable will be reclaimed by Rust code so does not need freeing
    local subtable = tables.export(value)
    loverust.tables_add_table(self.table, subtable.table)
  end
end

-- Adds a key - value pair to the hash map part of the Rust Table
function Table.put(self, key, value)
  if type(key) == 'string' then
    if type(value) == 'string' then
      loverust.tables_put_string_string(self.table, key, value)
    end
    if type(value) == 'number' then
      loverust.tables_put_string_number(self.table, key, value)
    end
    if type(value) == 'boolean' then
      loverust.tables_put_string_boolean(self.table, key, value)
    end
    if type(value) == 'table' then
      -- subtable will be reclaimed by Rust code so does not need freeing
      local subtable = tables.export(value)
      loverust.tables_put_string_table(self.table, key, subtable.table)
    end
  end
end

function Table.debug(self)
  loverust.tables_debug(self.table)
end

-- Frees the reference to a Rust Table.
function Table.free(self)
  loverust.tables_free_table(self.table)
  self = {}
end

return tables
