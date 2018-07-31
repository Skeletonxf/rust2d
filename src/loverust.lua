local ffi = require 'ffi'
-- This should be expanded to look for dll files when not
-- running on a POSIX system.

-- Loads the dynamic library
return ffi.load('./target/release/libloverust.so')
