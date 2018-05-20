This is a lot of trial and error in creating a template from which love2d lua code can call Rust to do the heavy lifting.

Love2d is "a framework for making 2D games in the Lua programming language". This project is attempting to use Rust, and the Luajit ffi to make calls to Rust code. The aim is to be able to write a game using Lua for the graphics and handling the update step in Rust. Calling functions already works but passing data from Lua to C to Rust and then back to C and then Lua is the main challenge. Rust, unlike C or Lua has a package manager from which we can access a huge number of libraries very easily. Exposing these to Lua to should be able to save a lot of effort.

## State of progress

What works
- Building the .so file with Cargo to call from Lua
- Running the 'game' with `love .` after the .so file is present
- Calling Rust functions with no return type or no arguments
- Passing numbers to and back from Rust
- Passing strings to and back from Rust
- Recieving arrays in Rust
- Passing custom structs to and back from Rust (should be able to extend much further)
- Passing [Rust objects](http://jakegoulding.com/rust-ffi-omnibus/objects/) to C and back
- Passing [arrays](https://github.com/Skeletonxf/rust2d/blob/master/src/arrays.rs) back from Rust to C

What does not yet work
- Passing arrays from C to Rust and modifying them rather than returning
- Passing arbitrary lua tables to and back from Rust (can already be done if serialising the table to a string both ways and using a [lua library](https://crates.io/search?q=lua) from the Rust side to convert back and then inspect but this will not be performant at all)
- A stable interface to use in real projects

## Examples
- Small [pong game](https://github.com/Skeletonxf/rust2d/tree/pong) using Rust for the update function and then lots of observers to draw it in love2d.
