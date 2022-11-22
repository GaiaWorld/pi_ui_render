cd ../
wasm-pack build --debug  --target web --out-dir pkg_debug --out-name wasm_engine
node build/build_wasm.js pkg_debug wasm_engine
pause;