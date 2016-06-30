#!/bin/bash

echo ""
echo "=== Checking Rust style with rustfmt... =============="

ERROR=0
for f in $(find . -regex '.+src/.+\.rs'); do
    # jesus I'm sorry, but I couldn't find a better way to use rustfmt :/
    if [ $(rustfmt --write-mode=checkstyle --skip-children $f | grep 'error' | wc -l) != 0 ]; then
        echo "! incorrectly formatted: $f"
        ERROR=1
    fi
done

test $ERROR == 0
