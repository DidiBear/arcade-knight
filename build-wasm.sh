mkdir -p wasm/build
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/arcade-knight.wasm wasm/build
cp -r wasm/static/* wasm/build
cp -r resources wasm/build
