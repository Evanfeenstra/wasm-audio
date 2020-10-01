import('./pkg/wasm_audio').then(module => {
    module.default('/wasm/pkg/wasm_audio_bg.wasm').then(()=>{
        // module.greet()
        // console.log(module)
        window.module = module
    })
})
.catch(console.error);

