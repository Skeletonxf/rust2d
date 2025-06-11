

# Rust2d

**Rust2d** is a utility library that allows [Love2D](https://love2d.org/) (a popular Lua-based 2D game framework) to offload preformance critical code from Lua to Rust. By compiling Rust code as a DLL and calling it from Lua via LuaJIT’s FFI, Rust2d brings the speed and extensive crate ecosystem of Rust to your Lua games and applications.

> **Why Rust?**  
> Rust offers a modern language with a powerful package manager (Cargo), memory safety, and access to a huge ecosystem of libraries. By bridging Rust and Lua, you can write performance-critical code in Rust while keeping your game logic in Lua.

---

## Features

- **Seamless FFI Bridge:** Call Rust functions from Lua using LuaJIT’s FFI.
- **Ready-to-use Modules:** Utilities for arrays, strings, and tables(work-in-progress).
- **Love2D Focused, LuaJIT Compatible:** Designed for Love2D, but will work with any LuaJIT interpreter.
- **Easy Extensibility:** Add your own Rust modules via Cargo, and expose them to Lua.

---

## Example

- **[Pong Game Example](https://github.com/Skeletonxf/rust2d/tree/pong):**  
  Demonstrates using Rust for the update logic, with Love2D handling rendering and input.

---

## Modules

- **arrays:** Send and receive arrays between Lua and Rust.
- **strings:** Safely pass strings between Lua and Rust.
- **tables:** *(Work in progress)* Pass Lua tables to Rust and back.

**Primitive types** (numbers, booleans) can be sent freely without special wrapping.

---

## Library Structure

- Each Rust module (e.g., `arrays`) has a corresponding Lua module (`arrays.lua`) that wraps the FFI calls.
- `loverust.lua` is responsible for loading the Rust dynamic library(DLL) and exposing its functions to Lua.
- `main.lua` and `lib.rs` are used for testing and experimentation.

---

## Getting Started

1. **Clone the repository and build the Rust library:**
   
   - Open a terminal and run:
     
     ```
     git clone https://github.com/Skeletonxf/rust2d.git
     cd rust2d
     ```
   - Make sure your `Cargo.toml` has `[lib] crate-type = ["cdylib"]` for FFI compatibility.
   - Build the Rust dynamic library in release mode:
     
     ```
     cargo build --release
     ```
   - This will generate a dynamic library file (`loverust.dll` on Windows, `libloverust.so` on Linux, or `libloverust.dylib` on macOS) in `target/release/`.

2. **Copy the generated dynamic library** (`loverust.dll` on Windows, `libloverust.so` on Linux, `libloverust.dylib` on macOS) into your Love2D project directory.
   
   - Place the DLL/so/dylib file in the same folder as your `main.lua` or wherever your Love2D project expects to load native libraries.
   - If you are using a custom project structure, ensure the path in your Lua FFI loader matches the location of the library file.

3. **Use the Lua modules** to call Rust functions from your Love2D game.
   
   - In your Lua code, require the provided Lua modules (e.g., `loverust.lua`, `arrays.lua`, `strings.lua`).
   - These modules use LuaJIT's FFI to load and call functions from the Rust library.
   - Example usage in `main.lua`:
     
     ```
     local loverust = require 'loverust'
     loverust.hello()
     print(loverust.add_two_numbers(2, 3))
     ```

---

## Contributing

Contributions are welcome! 

---

## Resources & References

These resources were invaluable in creating Rust2d:

- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Rust Language Tutorial](https://doc.rust-lang.org/tutorial.html)
- [LuaJIT FFI Documentation](http://luajit.org/ext_ffi.html)

---

## License

[MIT](LICENSE)

---
