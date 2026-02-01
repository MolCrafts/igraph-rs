use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "linux" && target_os != "macos" {
        panic!(
            "igraph-sys only supports Linux and macOS, got target OS: {}",
            target_os
        );
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let igraph_src = manifest_dir.join("../external/igraph");

    // Build igraph from bundled source via cmake.
    let dst = cmake::Config::new(&igraph_src)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("IGRAPH_ENABLE_TLS", "ON")
        .define("IGRAPH_ENABLE_LTO", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=igraph");

    // Link C++ standard library (igraph uses some C++ internally).
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=c++");
        // igraph uses LAPACK/BLAS via the Accelerate framework on macOS.
        println!("cargo:rustc-link-lib=framework=Accelerate");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=blas");
    }

    // Generate bindings from the installed igraph headers.
    let include_dir = dst.join("include").join("igraph");
    let header = include_dir.join("igraph.h");

    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .layout_tests(false)
        .generate_comments(false)
        .generate()
        .expect("failed to generate igraph bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings to OUT_DIR");

    // When the `regenerate` feature is enabled, also update the committed copy.
    if cfg!(feature = "regenerate") {
        bindings
            .write_to_file(manifest_dir.join("src/bindings.rs"))
            .expect("failed to write bindings to src/bindings.rs");
    }

    println!("cargo:rerun-if-changed=../external/igraph");
}
