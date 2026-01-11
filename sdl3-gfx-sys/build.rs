use std::path::PathBuf;

fn main() {
    let source_root = PathBuf::from("SDL3_gfx");

    let ctx = sdl3_build::BuildContext {
        lib_name: "SDL3_gfx".into(),
        package_name: "sdl3_gfx".into(),
        static_link: std::env::var("CARGO_FEATURE_STATIC_LINK").is_ok(),
        source_root,
    };

    let include_paths = sdl3_build::compile::prepare_library(&ctx);

    let config = sdl3_build::generate::BindingsConfig {
        wrapper_header: "wrapper.h".into(),
        include_paths,

        allowlist: vec![
            "^SDL_.*".into(),    // Standard
            "^SDL3_.*".into(),   // GFX-specific versioning
            "^pixel.*".into(),   // pixelColor, pixelRGBA
            "^h?vline.*".into(), // hline, vline
            "^rectangle.*".into(),
            "^box.*".into(),
            "^circle.*".into(),
            "^arc.*".into(),
            "^ellipse.*".into(),
            "^pie.*".into(),
            "^trigon.*".into(),
            "^polygon.*".into(),
            "^bezier.*".into(),
            "^character.*".into(),
            "^string.*".into(),
            "^gfx.*".into(), // gfxPrimitivesSetFont
            "^FPS.*".into(), // FPSmanager
            "^rotozoom.*".into(),
            "^zoom.*".into(),
            "^shrink.*".into(),
            "^rotate.*".into(),
        ],

        blocklist: vec![
            "SDL_Surface".into(),
            "SDL_Renderer".into(),
            "SDL_Rect".into(),
            "SDL_FPoint".into(),
            "SDL_version".into(),
        ],

        allowlist_files: vec![
            ".*SDL3_gfxPrimitives.h".into(),
            ".*SDL3_framerate.h".into(),
            ".*SDL3_rotozoom.h".into(),
            ".*SDL3_imageFilter.h".into(),
        ],

        raw_lines: vec!["use sdl3_sys::*;".into()],
    };

    sdl3_build::generate::generate_bindings(config);
}
