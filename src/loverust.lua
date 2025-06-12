local ffi = require 'ffi'
local libname

-- Uses LuaJIT to detect the operating system, which means it can be used in other LuaJIT environments.
if package.config:sub(1,1) == '\\' then -- Windows
    libname = './target/release/loverust.dll'
elseif jit.os == 'OSX' then -- macOS
    libname = './target/release/libloverust.dylib'
else -- Unix-like systems (Linux, etc.)
    libname = './target/release/libloverust.so'
end

-- Fallbacks and error handling
local ok, lib = pcall(ffi.load, libname)
if not ok then
    error(
        ("[loverust.lua] Failed to load Rust library: '%s'\n" ..
         "Make sure you have built the library with 'cargo build --release'\n" ..
         "and that the file exists at the specified location.\n" ..
         "Original error: %s"):format(libname, tostring(lib)),
        2
    )
end

return lib