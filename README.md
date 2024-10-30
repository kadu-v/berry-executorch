# berry-executorch

<p align="center">
    <img src="resource/image/logo.jpeg" alt="berry-executorch" width="200"　height="200">
</p>


This is the **simplest** Rust binding for the [executorch](https://pytorch.org/executorch-overview)

> [!IMPORTANT]
> This crate is under development and is not yet ready for production use.

## How to use
### General
1. Download the prebuilt executorch library from the [executorch-prebuilt](https://github.com/kadu-v/pre-built-executorch) website.
    ```bash
    #  In the root of the project
    $ wget https://github.com/kadu-v/pre-built-executorch/releases/download/v0.4.0/release.zip
    $ unzip release.zip
    ```

2. Set the `EXECUTORCH_HOME` environment variable to the `.cargo/config.toml` of your crate.
    ```toml
    [env]
    EXECUTORCH_HOME = "/path/to/executorch-prebuilt"
    ```

3. Add the following to your `Cargo.toml`
    ```toml
    [features]
    apple = ["executorch/apple"]
    android = ["executorch/android"]

    [dependencies]
    executorch = { git = "git@github.com:kadu-v/berry-executorch.git", version = "0.1.0" }
    ```

### iOS and Apple Silicon
1. Add the following to the `build.rs` of your crate.
    ```rust
    fn main() {
        #[cfg(feature = "apple")]
        {
            println!("cargo:rustc-link-arg=-fapple-link-rtlib");
            /* ---------- Foundation framework ------------------------------------ */
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Foundation");

            /* ---------- Metal backend frmework ---------------------------------- */
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=MetalPerformanceShaders");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=MetalPerformanceShadersGraph");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Metal");

            /* ---------- CoreML backend framework -------------------------------- */
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=CoreML");
            println!("cargo:rustc-link-arg=-framework");
            println!("cargo:rustc-link-arg=Accelerate");
            println!("cargo:rustc-link-lib=sqlite3");
        }
    }
    ```

### Android
1. Make `android.env` for android builds. Example:
    ```.env
    CXX = /path/to/android/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang++
    CC = /path/to/android/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang
    AR = /path/to/android/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
    ANDROID_NDK_HOME = /Users/user-name/Library/Android/sdk/ndk/28.0.12433566
    ANDROID_MIN_API_LEVEL = 35
    ```

    > [!TIPS]
    > For example, my `android.env` file looks like this:
    > ```env
    > CXX = /Users/kikemori/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/?bin/aarch64-linux-android35-clang++
    > CC = /Users/kikemori/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang
    > AR = /Users/kikemori/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
    > ANDROID_NDK_HOME = /Users/kikemori/Library/Android/sdk/ndk/28.0.12433566
    > ANDROID_MIN_API_LEVEL = 35
    > ```

2. Add the path of the `android.env` and linker to the `.cargo/config.toml`
    ```diff
    [env]
    EXECUTORCH_HOME = "/path/to/executorch-prebuilt"
    + ANDROID_ENV_PATH = "/path/to/android.env"

    [target.aarch64-linux-android]
    linker = "/path/to/android/ndk/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang++"
    ```

## How to develop

1. Download the prebuilt executorch library from the [executorch-prebuilt](https://github.com/kadu-v/pre-built-executorch) website.
    ```bash
    #  In the root of the project
    $ wget https://github.com/kadu-v/pre-built-executorch/releases/download/v0.4.0/release.zip
    $ unzip release.zip
    ```

2. Export Executorch home
    ```bash
    # Set the executorch home to the above extracted directory
    $ export EXECUTORCH_HOME=/path/to/executorch-prebuilt
    ```

3. Clone the repository
    ```bash
    $ git clone git@github.com:kadu-v/berry-executorch.git
    ```

4. Test the binding on android and ios
    ```bash
    # Test for the android feature
    $ cargo dingphy -d android test --features android --target aarch64-linux-android
    # Test for the ios
    $ cargo dinghy -d iphone test --features apple --target aarch64-apple-ios
    ```