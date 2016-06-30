#!/bin/bash

# Exit script on the first error
set -o errexit -o nounset

MY_PATH="`dirname \"$0\"`"

# basic style check
$MY_PATH/check-basic-style.sh

# basic style check
$MY_PATH/check-rustfmt.sh

# basic style check
$MY_PATH/build-all.sh

# basic style check
$MY_PATH/test-all.sh


echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
echo "+              Everything is fine!                 +"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
