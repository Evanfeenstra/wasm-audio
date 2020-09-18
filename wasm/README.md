
wasm-pack new wasm-audio
cd wasm-audio

wasm-pack build --target web

rollup ./pkg/wasm_audio.js --format iife --file ./pkg/bundle.js
