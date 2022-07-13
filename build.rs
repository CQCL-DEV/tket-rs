use conan::*;

use std::path::{Path, PathBuf};
use std::{env, process::Command};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let conan_profile = format!("tket-rs-{}-{}", target_os, target_arch);

    // Command::new("conan")
    //     .args(&["profile", "new", &conan_profile, "--detect"])
    //     .spawn()
    //     .expect("failed to create new profile");

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
        .filter_map(|dep| dep.get_include_dir().map(|dir| PathBuf::from(dir)))
        .collect();
    let mut loader_s = String::new();
    for c in build_info
        .dependencies()
        .iter()
        .filter_map(|dep| dep.get_library_dir().map(|dir| PathBuf::from(dir)))
    {
        println!("cargo:rustc-link-search={}", c.display());
        loader_s.push_str(&format!(":{}", c.display())[..]);
    }

    // modifiyng LD library path is bad actually
    // either put the libraries in target/ (in which case they will be added to
    // the loader path automatically)
    // or build tket statically and link that (preferred)
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{}",
        loader_s
    );

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
        .opt_level(1)
        .compile("tket-rs");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rustc-link-lib=tket-Circuit");
    println!("cargo:rustc-link-lib=tket-Ops");
    println!("cargo:rustc-link-lib=tket-OpType");
    println!("cargo:rustc-link-lib=tket-Gate");
    println!("cargo:rustc-link-lib=tket-Utils");
    println!("cargo:rustc-link-lib=tklog");
}
