#[allow(unused_imports)]
use dotenvy::from_filename_override;
use std::{env, path::Path};

fn main() {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let current_path = Path::new(&manifest_dir);
    #[cfg(feature = "android")]
    {
        from_filename_override("android.env")
            .expect("Failed to load android.env");
    }

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/c_interface.cpp")
        .include("src/cpp")
        .include("third_party/executorch-lib/include")
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
        "cargo:rustc-link-search={}/third_party/executorch-lib/{}/lib",
        current_path.display(),
        target_triple,
    );

    #[allow(unused_mut)]
    let mut libs = vec![
        "extension_tensor",
        "extension_module_static",
        "extension_data_loader",
        "executorch_no_prim_ops",
    ];

    #[rustfmt::skip]
    #[allow(unused_mut)]
    let mut whole_archive_libs = vec![
        "executorch", // Why should this library be linked as a whole archive?
        "portable_ops_lib",
        "portable_kernels",
    ];
    /* ---------- Common extra library configuration ---------------------- */
    #[cfg(feature = "xnnpack")]
    {
        libs.push("xnnpack_backend");
        libs.push("XNNPACK");
        libs.push("pthreadpool");
        libs.push("cpuinfo");
    }

    /* ---------- MacOS, iOS extra library configuration ---------------------- */
    #[cfg(feature = "apple")]
    {
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=Foundation");
        println!("cargo:rustc-link-arg=-fapple-link-rtlib");
    }

    /* Metal backend configuration */
    #[cfg(feature = "mps")]
    {
        whole_archive_libs.push("mpsdelegate");
        println!("cargo:rustc-link-arg=-weak_framework");
        println!("cargo:rustc-link-arg=MetalPerformanceShaders");
        println!("cargo:rustc-link-arg=-weak_framework");
        println!("cargo:rustc-link-arg=MetalPerformanceShadersGraph");
        println!("cargo:rustc-link-arg=-weak_framework");
        println!("cargo:rustc-link-arg=Metal");
    }

    /* CoreML backend configuration */
    #[cfg(feature = "coreml")]
    {
        whole_archive_libs.push("coremldelegate");
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=CoreML");
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=Accelerate");
        println!("cargo:rustc-link-lib=sqlite3");
    }

    /* ---------- Android extra library configuration ---------------------- */
    #[cfg(feature = "vulkan")]
    {
        libs.push("vulkan_backend");
    }

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    for lib in whole_archive_libs {
        println!("cargo:rustc-link-lib=static:+whole-archive={}", lib);
    }
}
