#!/usr/bin/env bash

rm -rf target

archs=("aarch64-unknown-linux-gnu" "x86_64-unknown-linux-gnu")

if [ "$(uname)" == "Darwin" ]; then
    archs+=("aarch64-apple-darwin" "x86_64-apple-darwin")
fi

for cmd in ${archs[@]};
do
    echo "Building for $cmd"
    cross build --target $cmd -r --bins
done
