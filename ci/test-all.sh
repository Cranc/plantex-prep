#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

for crate in base client server plantex; do
    echo "Building $crate..."
    cargo build -p $crate
    echo "Testing $crate..."
    cargo test -p $crate
done
