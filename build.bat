echo Building rust...
cargo build --release --lib --target wasm32-unknown-unknown

cd target
cd wasm32-unknown-unknown
cd release
wasm-bindgen eframe_template.wasm --out-dir "output" --no-modules --no-typescript

cd ..
cd ..
cd ..

copy /Y target\wasm32-unknown-unknown\release\output\eframe_template_bg.wasm eframe_template_bg.wasm
copy /Y target\wasm32-unknown-unknown\release\output\eframe_template.js eframe_template.js
