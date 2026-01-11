# sdl3-bindgen

FFI bindings for SDL3, using `bindgen` crate.

# Development

To build, make sure you have `cmake` installed, along with `clang` if building from source.

## Adding libraries

Create a new crate, set up `build.rs` (check the existing library crates for examples) and make sure to set up a submodule for the library repository. Then, add the crate to the workspace's `Cargo.toml`