local ffi = require 'ffi'
local libname

if jit.os == 'Windows' then
    libname = './loverust.dll'
elseif jit.os == 'OSX' then
    libname = './libloverust.dylib'
else -- Assumes Unix-like systems (Linux, etc.)
    libname = './libloverust.so'
end

-- Fallbacks and error handling
local ok, lib = pcall(ffi.load, libname)
if not ok then
    error(
        ("[loverust.lua] Failed to load Rust library: '%s'\n" ..
         "1. Make sure you have run the ./run.sh script to build the library.\n" ..
         "2. Ensure the library file is in the project's root directory.\n" ..
         "Original error: %s"):format(libname, tostring(lib)),
        2
    )
end

return lib
