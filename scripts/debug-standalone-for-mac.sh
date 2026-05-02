#!/bin/bash
package_name="thorgend"
move_from="./target/bundled/$package_name.vst3"
move_to="/Library/Audio/Plug-Ins/VST3/Thorgend.vst3"

cd nih-plug
cargo xtask bundle $package_name --release
target/bundled/$package_name.app/Contents/MacOS/$package_name  -r 44100