cd ../
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name wasm_engine
"C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-edb52840a79ee109\\wasm-bindgen.exe" "E:\\wasm_engine\\target\\wasm32-unknown-unknown\\release\\wasm_engine.wasm" "--out-dir" "E:\\wasm_engine\\pkg_profiling" "--typescript" "--target" "web" "--out-name" "wasm_engine"
node build/build_wasm.js pkg_profiling wasm_engine
pause;
