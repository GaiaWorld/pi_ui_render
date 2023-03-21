cd ../
wasm-pack build --release  --target web --out-dir pkg --out-name wasm_engine
node build/build_wasm.js pkg wasm_engine
pause;