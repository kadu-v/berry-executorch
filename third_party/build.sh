#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
EXECUTORCH_DIR=$SCRIPT_DIR/executorch
BUILD_DIR=$EXECUTORCH_DIR/cmake-out
BUILD_MODE=Release
set -ex


###############################################################################
# Auxiliary functions                                                         #
###############################################################################
function usage() {
    echo "--target: target architecture (e.g., aarch64-unknown-linux-gnu)"
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
elif [[ ! -z $TARGET_TRIPLE ]]; then
    println "Unsupported target architecture: $TARGET_TRIPLE"
    exit 1
fi

if [[ ! -z $CLEAN ]]; then
    println "Cleaning the build directory: $BUILD_DIR"
    rm -rf $BUILD_DIR
fi