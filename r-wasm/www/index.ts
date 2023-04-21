import init, {greet} from 'r-wasm'

init().then((_exports) => {
    greet('WebAssembly')
})

