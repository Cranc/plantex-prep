#!/bin/bash

# This file needs to be run from the git root directory!


echo "Testing 'common'..."
cd common
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
