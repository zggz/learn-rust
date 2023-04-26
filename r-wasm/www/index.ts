import init, {greet} from '../pkg/r_wasm.js'

init().then((_exports) => {
    greet('WebAssembly')
})

