#!/bin/bash
set -e

# Set variables
VERSION=$1

# Update Cargo version
sed -i "s/^version = .*$/version = \"$VERSION\"/" Cargo.toml
