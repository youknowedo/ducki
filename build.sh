#!/usr/bin/env bash

rm -rf target

for cmd in "aarch64-unknown-linux-gnu" "x86_64-unknown-linux-gnu" "aarch64-apple-darwin" "x86_64-apple-darwin"
do
    echo "Building for $cmd"
    cross build --target $cmd -r --bins
done
