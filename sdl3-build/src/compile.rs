use crate::BuildContext;
use std::path::PathBuf;

/// Tries to find the library via pkg-config/vcpkg, or falls back to building from source via CMake.
/// Returns the include paths to be used by bindgen.
pub fn prepare_library(ctx: &BuildContext) -> Vec<PathBuf> {
    // pkg-config
    if !ctx.static_link
        && let Ok(lib) = pkg_config::Config::new()
            .atleast_version("3.0.0")
            .probe(&ctx.package_name)
    {
        println!("cargo:rerun-if-changed=build.rs");
        return lib.include_paths;
    }

    // vcpkg
    if !ctx.static_link
        && cfg!(target_env = "msvc")
        && let Ok(lib) = vcpkg::find_package(&ctx.package_name)
    {
        println!("cargo:rerun-if-changed=build.rs");
        return lib.include_paths;
    }

    // Build from source using CMake
    build_from_source(ctx)
}

fn build_from_source(ctx: &BuildContext) -> Vec<PathBuf> {
    println!("cargo:rerun-if-changed={}", ctx.source_root.display());

    let mut cfg = cmake::Config::new(&ctx.source_root);

    cfg.define("SDL_SHARED", if ctx.static_link { "OFF" } else { "ON" });
    cfg.define("SDL_STATIC", if ctx.static_link { "ON" } else { "OFF" });
    cfg.define("SDL_TESTS", "OFF");
    cfg.define("SDL_EXAMPLES", "OFF");

    let dst = cfg.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib={}", ctx.lib_name);

    vec![dst.join("include")]
}
