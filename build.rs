#[allow(unused_imports)]
use dotenvy::from_filename_override;
#[allow(unused_imports)]
use std::{env, path::Path, path::PathBuf};

#[cfg(target_os = "macos")]
#[allow(dead_code)]
const TARGET_OS: &str = "darwin";

#[cfg(target_os = "linux")]
#[allow(dead_code)]
const TARGET_OS: &str = "linux";

fn main() {
    let manifest_dir_path =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let manifest_dir_path = Path::new(&manifest_dir_path);

    #[allow(unused_variables)]
    let target_triple = env::var("TARGET").unwrap();

    let profile = env::var("PROFILE").unwrap();
    let executorch_home =
        if let Some(executorch_home) = env::var("EXECUTORCH_HOME").ok() {
            executorch_home
        } else {
            Path::new(&manifest_dir_path)
                .join("executorch-prebuilt")
                .display()
                .to_string()
        };

    /* ------------------------------------------------------------------------
     * Compile C Interface for Executorch used in Rust
     * ------------------------------------------------------------------------ */
    let mut builder = &mut cc::Build::new();
    builder = builder
        .cpp(true)
        .file("src/cpp/c_interface.cpp")
        .include("src/cpp")
        .include(format!("{}/include", executorch_home))
        .flag("-std=c++17");

    #[cfg(feature = "android")]
    {
        let android_env_path =
            if let Some(android_env_path) = env::var("ANDROID_ENV_PATH").ok() {
                PathBuf::from(&android_env_path)
            } else {
                Path::new(&manifest_dir_path).join("android.env")
            };
        // Check the android.env file exists
        android_env_path.try_exists().expect(
            "android.env is not found, so please create it from android.env.sample and export \"ANDROID_ENV_PATH\" environment variable",
        );

        from_filename_override(android_env_path)
            .expect(&format!("Failed to load {}", manifest_dir_path.display()));

        // check the ANDROID_NDK_HOME environment variable
        let ndk_home = env::var("ANDROID_NDK_HOME").expect(
            "ANDROID_NDK_HOME is not set, so please set it from android.env",
        );
        let ndk_home = Path::new(&ndk_home);

        let min_api_level = env::var("ANDROID_MIN_API_LEVEL").expect(
        "ANDROID_MIN_API_LEVEL is not set, so please set it from android.env",
        );
        let min_api_level = min_api_level
            .parse::<u32>()
            .expect("Failed to parse ANDROID_MIN_API_LEVEL as u32");

        let sysroot_target_path = ndk_home.join(format!(
            "toolchains/llvm/prebuilt/{}-x86_64/sysroot",
            TARGET_OS
        ));

        if target_triple != "aarch64-linux-android" {
            panic!("Only aarch64-linux-android target is supported");
        }

        let target_lib_path =
            sysroot_target_path.join("usr/lib").join(&target_triple);
        let platform_lib_path = sysroot_target_path
            .join("usr/lib")
            .join("aarch64-linux-android")
            .join(min_api_level.to_string());
        let include_path = sysroot_target_path.join("usr/include");

        /* **IMPORTANT**
         * 以下のパスの順番はapi level に対応した ライブラリのパス -> NDK のパス にすること
         * 出ないと実行時にエラーが発生する
         * `library "libc++_shared.so" not found: needed by main executable`
         * */
        println!("cargo:rustc-link-search={}", platform_lib_path.display());
        println!("cargo:rustc-link-search={}", target_lib_path.display());
        println!("cargo:include={}", include_path.display());

        builder = builder
            .flag("-fexceptions")
            .flag("-frtti")
            .cpp_link_stdlib("c++_static");
    }

    /* Compile C++ */
    builder.compile("c_interface");

    println!("cargo:rerun-if-changed=cpp/src/c_interface.cpp");
    println!("cargo:rerun-if-changed=cpp/include/c_interface.h");
    println!(
        "cargo:rerun-if-changed={}/build.rs",
        manifest_dir_path.display()
    );

    /* ------------------------------------------------------------------------
     * Basic Linking Configuration
     * ------------------------------------------------------------------------ */
    println!(
        "cargo:rustc-link-search={}/{}/{}/lib",
        executorch_home, target_triple, profile
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
        whole_archive_libs.push("xnnpack_backend");
        whole_archive_libs.push("XNNPACK");
        whole_archive_libs.push("pthreadpool");
        whole_archive_libs.push("cpuinfo");
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
    #[cfg(feature = "android")]
    {
        println!("cargo:rustc-link-lib=static=c++abi");
    }
    #[cfg(feature = "vulkan")]
    {
        whole_archive_libs.push("vulkan_backend");
    }

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    for lib in whole_archive_libs {
        println!("cargo:rustc-link-lib=static:+whole-archive={}", lib);
    }

    /* ------------------------------------------------------------------------
     * Rerun build.rs if the following files are changed
     * ------------------------------------------------------------------------ */
    // C Interface
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cpp/c_interface.cpp");
    println!("cargo:rerun-if-changed=src/cpp/c_interface.h");

    // Executorch configurations
    println!("cargo:rerun-if-env-changed=EXECUTORCH_HOME");
    println!("cargo:rerun-if-changed=executorch-prebuilt");

    // Android configurations
    println!("cargo:rerun-if-env-changed=ANDROID_ENV_PATH");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=ANDROID_MIN_API_LEVEL");
    println!("cargo:rerun-if-changed=android.env");
}
