use std::{env, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let current_path = Path::new(&manifest_dir);

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/c_interface.cpp")
        .include("src/cpp")
        .include("third_party/executorch_lib/include")
        .flag("-std=c++17")
        .compile("c_interface");
    println!("cargo:rerun-if-changed=cpp/src/c_interface.cpp");
    println!("cargo:rerun-if-changed=cpp/include/c_interface.h");
    println!("cargo:rerun-if-changed={}/build.rs", current_path.display());

    /* ------------------------------------------------------------------------
    * Basic Linking Configuration
    --------------------------------------------------------------------------- */
    let target_triple = env::var("TARGET").unwrap();
    println!(
        "cargo:rustc-link-search={}/third_party/executorch_lib/{}/lib",
        current_path.display(),
        target_triple,
    );
    let libs = vec![
        "executorch",
        "extension_data_loader",
        "extension_tensor",
        "extension_module_static",
        "executorch_no_prim_ops",
        // "extension_runner_util",
        // "extension_threadpool",
        // "pthreadpool",
    ];
    for lib in libs {
        println!("cargo:rustc-link-lib=static:+whole-archive={}", lib);
    }
    let whole_archive_libs = vec!["portable_ops_lib", "portable_kernels"];
    for lib in whole_archive_libs {
        println!("cargo:rustc-link-lib=static:+whole-archive={}", lib);
    }
}
