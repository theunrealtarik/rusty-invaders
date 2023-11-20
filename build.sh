#!/bin/bash

platforms="x86_64-apple-darwin x86_64-pc-windows-gnu x86_64-unknown-linux-gnu"

for platform in $platforms; do
  cargo build --target=$platform --release || echo "Failed to build for platform $platform"
done

if [ $? -ne 0 ]; then
  echo "Overall build failed"
  exit 1
fi

exit 0