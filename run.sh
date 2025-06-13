#!/bin/bash
set -e
set -u  # Treats unset variables as errors
set -o pipefail  # Fail if any command in a pipeline fails

# Uncomment the following line to enable Rust debug logging
# export RUST_LOG=debug

# Uncomment to log Love2D output to a file for debugging
# LOVE_LOG="love2d.log"
# love . > "$LOVE_LOG" 2>&1 || { echo "Love2D failed to start. See $LOVE_LOG for details."; exit 1; }

if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

echo "Building Rust library..."
cargo build --release

case "$(uname | tr '[:upper:]' '[:lower:]')" in
    darwin)
        LIB="./target/release/libloverust.dylib"
        ;;
    linux)
        LIB="./target/release/libloverust.so"
        ;;
    mingw*|msys*|cygwin*|windows_nt)
        LIB="./target/release/loverust.dll"
        ;;
    *)
        echo "Unknown OS: $(uname -a)"
        echo "Unknown OS. Please manually copy the dynamic file from target/release/ to your project directory that contains main.lua."
        exit 1 
        ;;
esac

if [ ! -f "$LIB" ]; then
    echo "Build succeeded, but $LIB was not found."
    echo "Check your Cargo.toml and build output for errors."
    exit 1
fi

# Copy library to project directory for Love2D to find
cp "$LIB" .

if ! command -v love >/dev/null 2>&1; then
    echo "Love2D is not installed or not in your PATH."
    exit 1
fi

# Uncomment the following lines to run Lua Busted tests before launching Love2D
# if command -v busted >/dev/null 2>&1; then
#     echo "Running Lua Busted tests..."
#     busted || { echo "Some Lua tests failed."; exit 1; } # If you installed busted wuth luarocks, remember that luarocks installs modules as .bat files on Windows, so you may need to run `busted.bat` instead of `busted`.
# else
#     echo "Busted not found; skipping Lua tests."
# fi

echo "Launching Love2D..."
love . || { echo "Love2D found in path, but failed to start. Please check your Love2D installation."; exit 1; }
echo "Love2D exited."
