pub mod compile;
pub mod generate;

use std::path::PathBuf;

/// Context for the build, usually passed from the sys crate's build.rs
pub struct BuildContext {
    pub lib_name: String,
    pub package_name: String,
    pub static_link: bool,
    /// Path to the source submodule (e.g. "sdl3-src/SDL")
    pub source_root: PathBuf,
}
