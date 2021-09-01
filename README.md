# Rust2d

This is a work in progress library of utility code for creating a project where love2d Lua code can call Rust code to do the heavy lifting. Love2d is "a framework for making 2D games in the Lua programming language". This project provides Rust code compiled to a dynamic library which is then called via LuaJIT's FFI. The 'C' code that LuaJIT calls is then wrapped in lua code. Rust, unlike C or Lua has a (first class) package manager from which we can access a huge number of libraries very easily. Exposing these to Lua to should be able to save a lot of effort when writing games or other software. Although the project is focused on applications for use with the Love2d framework, the library code should run fine in any LuaJIT interpreter.

## Examples
- Small [pong game](https://github.com/Skeletonxf/rust2d/tree/pong) using Rust for the update function and then lots of observers to draw it in love2d.

# Modules

- **arrays** Code for sending and recieving arrays over FFI.
- **strings** Code for sending and recieving strings over FFI.
- **tables** Work in progress code for sending Lua tables over FFI.

## Non modules

These can be sent freely with no wrapping.

- **numbers**
- **booleans**

# Library

Each module written on the Rust side has a Lua module by the same name and location which is responsible for calling the Rust code. The only exceptions to this are testing code in main.lua, lib.rs and loverust.lua. The former 2 are where new library functionality is tested, and the latter is responsible for loading the Rust code to call from Lua.

# Notes

The Rust FFI omnibus, Rust tutorial and LuaJIT FFI tutorial pages have been extremely helpful for me in creating this project.

- http://jakegoulding.com/rust-ffi-omnibus/
- https://doc.rust-lang.org/tutorial.html
- http://luajit.org/ext_ffi.html
