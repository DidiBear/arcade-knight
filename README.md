# Arcade Knight

TODO: Play on itch.io !

TODO: Preview 
![preview](about/preview.png)]

A small arcade game made with Rust and the
[`macroquad`](https://github.com/not-fl3/macroquad) game engine.

## Build for WebAssembly

Globally add the build target for WebAssembly:

```shell
rustup target add wasm32-unknown-unknown
```

Build the game using: 

```shell
./build-wasm
```

This script compiles the `arcade-knight.wasm` program and copy all `resources`
and `wasm/static` files to the `wasm/build` folder. 

To run the game in the browser, you can install `basic-http-server` using:

```shell
cargo install basic-http-server
```

Then start the server using:

```shell
basic-http-server wasm/build
```