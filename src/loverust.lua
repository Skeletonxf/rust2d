local ffi = require 'ffi'
-- Windows
if love.system.getOS() == 'Windows' then
  return ffi.load('./target/release/loverust.dll')
end
-- POSIX
return ffi.load('./target/release/libloverust.so')
