#!/bin/bash
#
# This script generates the README.md file based on the rustdoc documentation of the crate.
#
# Note that this currently only supports the following types of links:
# - [Name](crate::...)
#
set -e

# Use "cargo rdme" to generate the README.md file
cargo rdme --force

# Replace `![alt][image]`` with `![alt](image)`. This is needed because cargo-embed-image requires the former syntax to generate rustdoc, but GitHub markdown requires the latter.
sed -i 's/!\[\([^]]*\)\]\[\([^]]*\)\]/![\1](\2)/g' README.md

echo Success
