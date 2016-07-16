#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

export RUSTFLAGS="--deny warnings"

echo "Building 'base'..."
cd base
cargo build -v
cd ..


echo "Building 'client'..."
cd client
cargo build -v
cd ..


echo "Building 'server'..."
cd server
cargo build -v
cd ..
