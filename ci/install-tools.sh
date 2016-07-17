#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH

if ! type travis-cargo > /dev/null; then
    echo ""
    echo "=== Installing travis-cargo ================"
    pip install 'travis-cargo<0.2' --user
fi

if ! type rustfmt > /dev/null; then
    echo ""
    echo "=== Installing rustfmt ==============="
    cargo install rustfmt
fi
