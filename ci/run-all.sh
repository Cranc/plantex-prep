#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

MY_PATH="`dirname \"$0\"`"

# install CI tools
$MY_PATH/install-tools.sh

# basic style check
$MY_PATH/check-basic-style.sh

# check that everything is formatted with rustfmt
$MY_PATH/check-rustfmt.sh

# check that everything compiles and all tests pass
$MY_PATH/test-all.sh

# generate and possibly upload docs to GitHub Pages
$MY_PATH/upload-docs.sh

echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
echo "+              Everything is fine!                 +"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
