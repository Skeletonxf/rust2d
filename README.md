

# Rust2d

**Rust2d** is a utility template that allows [Love2D](https://love2d.org/) (a popular Lua-based 2D game framework) to offload preformance critical code from Lua to Rust. By compiling Rust code as a DLL and calling it from Lua via LuaJIT’s FFI, Rust2d brings the speed and extensive crate ecosystem of Rust to your Lua games and applications.

> **Why Rust?**  
> Rust offers a modern language with a powerful package manager (Cargo), memory safety, and access to a huge ecosystem of libraries. By bridging Rust and Lua, you can write preformance critical code in Rust while keeping your game logic in Lua.

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

### 1. Clone the Repository

```sh
git clone https://github.com/Skeletonxf/rust2d.git
cd rust2d
```

---

### 2. Build and Run with the Automation Script

- **Use the provided `run.sh` script** to build the Rust library and launch Love2D in one step:

  ```sh
  ./run.sh
  ```

  - This script:
    - Compiles the Rust library in release mode using `cargo build --release`
    - Detects your operating system and confirms the correct dynamic library (`.dll`, `.so`, or `.dylib`) is present in `./target/release/`.
    - Launches Love2D, which is now ready to use your Rust modules.

  - **Note:**  
    If you are on non-WSL Windows, run this script from Git Bash or an MSYS2 Bash shell like **MingW64** for best compatibility. If you use PowerShell, invoke the script explicitly with `bash ./run.sh`.

---

### 3. How the Loader Works (`loverust.lua`)

- The `loverust.lua` module **automatically detects your OS** and loads the correct dynamic library from `./target/release/`.
- It includes **error handling**: if the library cannot be loaded, you’ll get a clear error message with troubleshooting tips.
- No need to manually copy the dynamic library or modify paths unless you change the project structure, or are using an incompatible operating system.

---

### 4. Using Rust Functions in Lua

- In your Love2D Lua code, require the Rust loader and modules as usual:

  ```lua
  local loverust = require 'loverust'
  print(loverust.hello())
  print(loverust.add_two_numbers(2, 3))
  ```

- The provided modules (`arrays.lua`, `strings.lua`, etc.) use LuaJIT FFI to call Rust functions seamlessly.

---

### 5. Troubleshooting

- **If you see errors about missing libraries:**  
  Ensure you have built the Rust code with `./run.sh` (or manually running `cargo build --release`) and are running the script from the project root.
- **If `love` is not found:**  
  Make sure Love2D is installed and its executable is in your system or environment PATH.

---

### **6. Custom Project Structures**

- If you move the Rust dynamic library or change your project structure, then update the path in `loverust.lua` accordingly.

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

This project is licensed under the MIT license ([LICENSE](LICENSE) or https://opensource.org/license/MIT).
See the license file for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
