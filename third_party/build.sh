#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
EXECUTORCH_DIR=$SCRIPT_DIR/executorch
BUILD_DIR=$EXECUTORCH_DIR/cmake-out
BUILD_MODE=Release


###############################################################################
# Auxiliary functions                                                         #
###############################################################################
function usage() {
    echo "Usage: $0 --target=<target> [--clean] [--mode=<mode>]"
    echo "--target: target architecture (e.g., aarch64-unknown-linux-gnu, apple-*)"
    echo "--clean: clean the build directory"
    echo "--mode: build mode (e.g., Release, Debug)"
}

function println() {
    echo "###############################################################################"
    echo "$1"
    echo "###############################################################################"
}


###############################################################################
# Parse command line arguments                                                #
###############################################################################
for arg in "$@"; do
    case $arg in
        --target=*) TARGET_TRIPLE="${arg#*=}";;
        --clean) CLEAN=1;;
        --mode=*) BUILD_MODE="${arg#*=}";;
        *)
        echo "Invalid argument: $arg"
        exit 1
        ;;
    esac
done


###############################################################################
# Main                                                                        #
###############################################################################

if [[ -z "$TARGET_TRIPLE" ]] && [[ -z "$CLEAN" ]]; then
    usage
    exit 1
fi

# Check the MacOS environment
if [[ $(uname) == "Darwin" ]]; then
    println "Checking python environment is activated"
    if [[ -z "$VIRTUAL_ENV" ]]; then
        echo "Please activate a python environment"
        exit 1
    fi

    println "Checking buck2 version"
    BUCK2_VERSION=$(cat ${SCRIPT_DIR}/executorch/.ci/docker/ci_commit_pins/buck2.txt)
    BUCK2_EXECUTABLE="${SCRIPT_DIR}/executorch/buck-out/buck2-aarch64-apple-darwin"
    if [[ ! -f "buck2/buck2-aarch64-apple-darwin" ]]; then
        wget "https://github.com/facebook/buck2/releases/download/${BUCK2_VERSION}/buck2-aarch64-apple-darwin.zst"
        unzstd buck2-aarch64-apple-darwin.zst
        chmod u+x buck2-aarch64-apple-darwin
        mv buck2-aarch64-apple-darwin $BUCK2_EXECUTABLE
    else
        echo "Buck2 already downloaded"
    fi
fi

if [[ $TARGET_TRIPLE == "aarch64-unknown-linux-gnu" ]]; then
    println "Building for aarch64-unknown-linux-gnu"

    # Check the host machine architecture
    if [[ $(uname -m) != "aarch64" ]]; then
        println "Host machine architecture is not aarch64"
        exit 1
    fi
    # Check the host machin os type
    if [[ $(uname) != "Linux" ]]; then
        println "Host machine OS is not Linux"
        exit 1
    fi

    # Build the project
    (
        cd $EXECUTORCH_DIR
        cmake . -B $BUILD_DIR \
            -DEXECUTORCH_BUILD_EXTENSION_TENSOR=ON \
            -DEXECUTORCH_BUILD_EXTENSION_MODULE=ON \
            -DEXECUTORCH_BUILD_EXTENSION_DATA_LOADER=ON \
            -DEXECUTORCH_BUILD_EXTENSION_RUNNER_UTIL=ON \
            -DEXECUTORCH_BUILD_EXECUTOR_RUNNER=OFF \
            -DCMAKE_BUILD_TYPE=$BUILD_MODE 
        cd $BUILD_DIR
        make -j4
        cmake --install . --prefix $EXECUTORCH_DIR/../lib/$TARGET_TRIPLE

        println "Extract all headers from executorch and copy them to the include directory"
        cd $EXECUTORCH_DIR
        mkdir -p $EXECUTORCH_DIR/../lib/include/executorch
        find . -name "*.h" -exec cp --parents {} $EXECUTORCH_DIR/../lib/include/executorch \;        
    )
elif [[ $TARGET_TRIPLE == "aarch64-*" ]]; then
    println "Building for ${TARGET_TRIPLE}"
    (
        cd $EXECUTORCH_DIR
        sed -i '' 's/set -euo pipefail//g' build/build_apple_frameworks.sh

        println "Building frameworks"
        ./build/build_apple_frameworks.sh \
            --output=cmake-out \
            --toolchain=third-party/ios-cmake/ios.toolchain.cmake \
            --buck2=$BUCK2_EXECUTABLE \
            --python=$(which python) \
            --coreml \
            --custom \
            --mps \
            --optimized \
            --portable \
            --quantized \
            --xnnpack
    )

    (
        println "Extract all headers from executorch and copy them to the include directory for \"aarch64-apple-darwin\""
        cd $BUILD_DIR

        cmake --install . --prefix $EXECUTORCH_DIR/../lib/aarch64-apple-darwin
    )

    (
        println "Extract all headers from executorch and copy them to the include directory for \"arm64-apple-ios\""
        cd $BUILD_DIR

        cmake --install . --prefix $EXECUTORCH_DIR/../lib/arm64-apple-ios
    )

    (
        println "Extract all headers from executorch and copy them to the include directory for \"arm64-apple-ios-sim\""
        cd $BUILD_DIR

        cmake --install . --prefix $EXECUTORCH_DIR/../lib/arm64-apple-ios-sim
    )
elif [[ ! -z $TARGET_TRIPLE ]]; then
    println "Unsupported target architecture: $TARGET_TRIPLE"
    exit 1
fi

if [[ ! -z $CLEAN ]]; then
    println "Cleaning the build directory: $BUILD_DIR"
    rm -rf $BUILD_DIR
fi