#!/bin/sh

# install tools
cargo install grcov
rustup component add llvm-tools-preview

#timestamp
directory_name="$(date '+%Y-%m-%d_%H-%M-%S')"

# build
export LLVM_PROFILE_FILE="target/coverage/${directory_name}/%p-%m.profraw"
export RUSTFLAGS="-Cinstrument-coverage"
cargo build

# run tests
cargo test --workspace

# generate report
grcov target/coverage --binary-path target/debug -s . -o "target/coverage/${directory_name}" --keep-only 'src/*' --output-types html,cobertura

#open report
open "./target/coverage/${directory_name}/html/index.html"