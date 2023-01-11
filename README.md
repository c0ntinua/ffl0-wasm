# ffl0-wasm

This is a simple version of ffl0 intended for Web Assembly runtimes (such as Wasmer). All graphics-like effects are obtained with ANSI escape codes. I have added a wasm file  `wasm-ffl0.wasm` above in case you want to use fflo without worrying about installing Rust.

To run wasm-ffl0 with its default settings, just install wasmer and type `wasmer wasm-ffl0.wasm`.

The default parameters, in order, are rows, cols, filters, span, flux. Increasing all but the last will slow down the program. Flux determines how often the filters are randomly changed and how often the state is randomized. You might use them this way: 
```wasmer wasm-ffl0.wasm 50 100 11 3 50```.

