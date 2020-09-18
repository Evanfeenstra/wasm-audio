import('./pkg/wasm_audio').then(module => {
    module.default('/wasm/pkg/wasm_audio_bg.wasm').then(()=>{
        module.greet()
    })
})
.catch(console.error);

