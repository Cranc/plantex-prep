#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH

if ! type ghp-import > /dev/null; then
    echo ""
    echo "=== Installing ghp-import ================"
    pip install ghp-import
fi

if ! type rustfmt > /dev/null; then
    echo ""
    echo "=== Installing rustfmt ==============="
    cargo install rustfmt
fi
