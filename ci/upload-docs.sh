#!/bin/bash

# Exit script on the first error
set -e

echo ""
echo "=== Generating documentation ================="
cargo doc

if [ "$TRAVIS_BRANCH" == "master" ]; then
    echo ""
    echo "=== Uploading docs ==============="
    export TRAVIS_CARGO_NIGHTLY_FEATURE=
    travis-cargo doc-upload
fi
