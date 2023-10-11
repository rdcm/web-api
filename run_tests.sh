#!/bin/bash

# install tools
cargo install grcov
rustup component add llvm-tools-preview

# create coverage directory
timestamp="$(date '+%Y-%m-%d_%H-%M-%S')"
directory_path="target/coverage/${timestamp}"

# build
export LLVM_PROFILE_FILE="${directory_path}/%p-%m.profraw"
export RUSTFLAGS="-Cinstrument-coverage"
cargo build --workspace

# run tests
cargo test --workspace

# move all *.profraw
find . -path "*/${timestamp}/*" -name '*.profraw' -exec mv {} "./${directory_path}" \;

# generate report
grcov "target/coverage/${timestamp}" --binary-path target/debug -s . -o "${directory_path}" --ignore "target/debug/*" --ignore "integrtion-tests/*" --output-types html

# open report
open "./${directory_path}/html/index.html"