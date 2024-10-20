# executorch-rs

This is a Rust binding for the [executorch](https://pytorch.org/executorch-overview)

## How to use
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

4. (Optional) Make `android.env` for android builds. Example:
    ```.env
    CXX = /Users/user-name/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang++
    CC = /Users/user-name/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android35-clang
    AR = /Users/user-name/Library/Android/sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
    ANDROID_NDK_HOME = /Users/user-name/Library/Android/sdk/ndk/28.0.12433566
    ANDROID_MIN_API_LEVEL = 35
    ```

3. Add the following to your `Cargo.toml`
    ```toml
    [features]
    apple = ["executorch/apple"]
    android = ["executorch/android"]

    [dependencies]
    executorch = { git = "git@github.com:kadu-v/executorch-rs.git", version = "0.1.0" }
    ```


## Hot to develop

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
    $ git clone git@github.com:kadu-v/executorch-rs.git
    ```

4. Test the binding
    ```bash
    # Test for the apple feature
    $ cargo test --features apple
    # Test for the android feature
    $ cargo test --features android 
    ```

5. Test the binding on android
    ```bash
    # Test for the android feature
    $ cargo dingphy -d android test --features android --target aarch64-linux-android
    # Test for the ios
    $ cargo dinghy -d iphone test --features apple --target aarch64-apple-ios
    ```