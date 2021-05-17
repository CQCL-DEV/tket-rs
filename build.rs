use conan::*;

use std::path::{Path, PathBuf};
use std::{env, fs::read_dir, process::Command};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let conan_profile = format!("tket-rs-{}-{}", target_os, target_arch);

    Command::new("conan")
        .args(&["profile", "new", &conan_profile, "--detect"])
        .spawn()
        .expect("failed to create new profile");

    let recipes = read_dir("./tket/recipes/").expect("recipes directory not found");
    for recipe in recipes {
        Command::new("conan")
            .arg("export")
            .arg(recipe.unwrap().path())
            .spawn()
            .expect("failed to export recipes");
    }

    let command = InstallCommandBuilder::new()
        .with_profile(&conan_profile)
        .build_policy(BuildPolicy::Missing)
        .recipe_path(Path::new("conanfile.txt"))
        .build();

    assert!(command.generate().is_some());
    let build_info = command
        .generate()
        .expect("failed to generate conan build info");

    let mut cxx_includes = Vec::new();

    let conan_includes = build_info
        .dependencies()
        .iter()
        .filter_map(|dep| dep.get_include_dir().map(|dir| PathBuf::from(dir)));

    for conan_include in conan_includes {
        let mut conan_include = conan_include.clone();
        // Patch the include path for eigen
        if conan_include.iter().position(|x| x == "eigen").is_some() {
            conan_include.push("eigen3");
        }
        println!("{:?}", conan_include);
        cxx_includes.push(conan_include);
    }

    // Allow extra custom C++
    cxx_includes.push(PathBuf::from("src/"));

    let mut build = autocxx_build::build("src/lib.rs", cxx_includes, &["-std=c++17"]).unwrap();
    build
        .define("SPDLOG_FMT_EXTERNAL", "ON")
        .define("BOOST_ALLOW_DEPRECATED_HEADERS", "ON")
        .flag("-std=c++17")
        .opt_level(1)
        .compile("tket-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
