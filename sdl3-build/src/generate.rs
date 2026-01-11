use std::env;
use std::path::PathBuf;

#[derive(Default)]
pub struct BindingsConfig {
    /// The main header file (e.g. "SDL.h" or "SDL3_gfx/SDL3_gfxPrimitives.h")
    pub wrapper_header: String,
    /// List of include paths to search
    pub include_paths: Vec<PathBuf>,
    /// Regex allowing specific items (functions, types, vars)
    pub allowlist: Vec<String>,
    /// Regex blocklisting items (useful if reusing types from another crate)
    pub blocklist: Vec<String>,
    /// Exact file paths to generate bindings for.
    /// If non-empty, only items defined in these files will be generated.
    pub allowlist_files: Vec<String>,
    /// Raw lines to prepend to the generated file (e.g. "use sdl3_sys::*;")
    pub raw_lines: Vec<String>,
}

pub fn generate_bindings(config: BindingsConfig) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = bindgen::Builder::default()
        .header(&config.wrapper_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false) // Enable if debugging ABI
        .generate_comments(true)
        .use_core()
        .ctypes_prefix("::core::ffi")
        .prepend_enum_name(false)
        // Handle complex macros that bindgen often skips
        .clang_arg("-DSDL_DISABLE_ANALYZE_MACROS");

    for path in config.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    // Apply Allow/Block lists
    for pattern in config.allowlist {
        builder = builder
            .allowlist_type(&pattern)
            .allowlist_function(&pattern)
            .allowlist_var(&pattern);
    }

    for pattern in config.blocklist {
        builder = builder
            .blocklist_type(&pattern)
            .blocklist_function(&pattern)
            .blocklist_item(&pattern);
    }

    for file_pattern in config.allowlist_files {
        builder = builder.allowlist_file(file_pattern);
    }

    for line in config.raw_lines {
        builder = builder.raw_line(line);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
