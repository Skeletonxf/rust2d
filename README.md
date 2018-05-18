This is a lot of trial and error in creating a template from which love2d lua code can call Rust to do the heavy lifting.

Love2d is "a framework for making 2D games in the Lua programming language". This project is attempting to use Rust, and the Luajit ffi to make calls to Rust code. The aim is to be able to write a game using Lua for the graphics and handling the update step in Rust. Calling functions already works but passing data from Lua to C to Rust and then back to C and then Lua is the main challenge. Rust, unlike C or Lua has a package manager from which we can access a huge number of libraries very easily. Exposing these to Lua to should be able to save a lot of effort.

What works
- building the .so file with Cargo to call from Lua
- running the 'game' with `love .` after the .so file is present
- calling Rust functions with no return type or no arguments
- passing numbers to and back from Rust
- Passing strings to and back from Rust
- recieving arrays in Rust
- passing custom structs to and back from Rust (should be able to extend much further)
- passing [Rust objects](http://jakegoulding.com/rust-ffi-omnibus/objects/) to C and back

What does not yet work
- passing arrays from C to Rust and modifying them rather than returning
- passing [arrays](https://stackoverflow.com/questions/49591678/is-it-possible-to-pass-arrays-from-rust-to-c) back from Rust to C
- passing arbitrary lua tables to and back from Rust

Examples
- Small [pong game](https://github.com/Skeletonxf/rust2d/tree/pong) using Rust for the update function and then lots of observers to draw it in love2d.
