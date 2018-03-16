
## Note!

To build for native just run `cargo run`

To build for the website you have to run:

```
export EMMAKEN_CFLAGS="-s USE_SDL=2"
cargo build --target=asmjs-unknown-emscripten
```

That `export` is important, as it tells Emscripten to use SDL2, which you should have installed on your computer!

All based on [this lovely blog post on Rust + SDL + `.wasm`](https://blog.fazibear.me/definitive-guide-to-rust-sdl-2-and-emscripten-93d707b22bbb).
