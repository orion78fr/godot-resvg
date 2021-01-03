#!/bin/bash

cargo build --release
cp target/release/*.dll addons/godot_resvg/
# linux -- cp target/release/*.so addons/godot_resvg/
# macos -- cp target/release/*.dylib addons/godot_resvg/
rm -r ../../Documents/godot/rustyGeo/addons/
cp -r addons/ ../../Documents/godot/rustyGeo

