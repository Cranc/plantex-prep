#!/bin/bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

echo "Testing 'base'..."
cd base
cargo test -v
cd ..


echo "Testing 'client'..."
cd client
cargo test -v
cd ..


echo "Testing 'server'..."
cd server
cargo test -v
cd ..
