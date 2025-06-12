#!/bin/bash
set -e

echo "Building Rust library..."
cargo build --release

case "$(uname)" in
    Darwin)
        LIB="./target/release/libloverust.dylib"
        ;;
    Linux)
        LIB="./target/release/libloverust.so"
        ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
        LIB="./target/release/loverust.dll"
        ;;
    *)
        echo "Unknown OS. Please manually copy the dynamic file from target/release/ to your project directory that contains main.lua."
        exit 1
        ;;
esac

if [ ! -f "$LIB" ]; then
    echo "Build succeeded, but $LIB was not found."
    echo "Check your Cargo.toml and build output for errors."
    exit 1
fi

echo "Launching Love2D..."
love .
echo "Love2D exited."
