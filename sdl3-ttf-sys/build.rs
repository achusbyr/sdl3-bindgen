use std::path::PathBuf;

fn main() {
    let source_root = PathBuf::from("SDL_ttf");

    let ctx = sdl3_build::BuildContext {
        lib_name: "SDL3_ttf".into(),
        package_name: "sdl3_ttf".into(),
        static_link: std::env::var("CARGO_FEATURE_STATIC_LINK").is_ok(),
        source_root,
    };

    let include_paths = sdl3_build::compile::prepare_library(&ctx);

    let config = sdl3_build::generate::BindingsConfig {
        wrapper_header: "wrapper.h".into(),
        include_paths,

        // 1. Allowlist: TTF specific
        allowlist: vec!["^TTF_.*".into()],

        // 2. Blocklist: No core SDL types
        blocklist: vec!["^SDL_.*".into()],

        // 3. Inject dependency
        raw_lines: vec!["use sdl3_sys::*;".into()],

        ..Default::default()
    };

    sdl3_build::generate::generate_bindings(config);
}
