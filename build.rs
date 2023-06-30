use conan::*;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let conan_profile = format!("tket-rs-{}-{}", target_os, target_arch);

    let command = InstallCommandBuilder::new()
        .with_profile(&conan_profile)
        .build_policy(BuildPolicy::Never)
        .recipe_path(Path::new("conanfile.txt"))
        .build();

    // assert!(command.generate().is_some());
    let build_info = command
        .generate()
        .expect("failed to generate conan build info");

    let mut cxx_includes: Vec<_> = build_info
        .dependencies()
        .iter()
        .filter_map(|dep| dep.get_include_dir().map(PathBuf::from))
        .collect();
    for c in build_info
        .dependencies()
        .iter()
        .filter_map(|dep| dep.get_library_dir().map(PathBuf::from))
    {
        println!("cargo:rustc-link-search={}", c.display());
        // loader_s.push_str(&format!(":{}", c.display())[..]);
    }

    // Allow extra custom C++
    cxx_includes.push(PathBuf::from("src/"));

    let mut build = autocxx_build::Builder::new("src/lib.rs", &cxx_includes)
        .extra_clang_args(&["-std=c++17", "-DSPDLOG_FMT_EXTERNAL=ON"])
        .build()
        .unwrap();
    build
        .define("SPDLOG_FMT_EXTERNAL", "ON")
        .define("BOOST_ALLOW_DEPRECATED_HEADERS", "ON")
        .file("src/unitary.cpp")
        .flag("-std=c++17")
        .flag("-w") // Ignore warnings from tket (and symengine)
        .opt_level(1)
        .compile("tket-rs");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rustc-link-lib=static=tket-Circuit");
    println!("cargo:rustc-link-lib=static=tket-Ops");
    println!("cargo:rustc-link-lib=static=tket-OpType");
    println!("cargo:rustc-link-lib=static=tket-Gate");
    println!("cargo:rustc-link-lib=static=tket-Utils");
    println!("cargo:rustc-link-lib=static=tklog");
    println!("cargo:rustc-link-lib=static=tkassert");
    println!("cargo:rustc-link-lib=static=tkrng");
    println!("cargo:rustc-link-lib=static=symengine");
    println!("cargo:rustc-link-lib=static=gmp");
}
