# n1

Landing Page for N-1 Labs, built with Leptos (Rust/WASM).

## Development

```
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve
```

## Build

```
trunk build --release --public-url "/n1/"
cargo run -p prerender --release -- dist
```

`trunk build` compiles the WASM frontend and assets into `dist/`. The `prerender`
binary then server-renders the app and injects it into `dist/index.html`, so the
shipped page has real content before the WASM hydrates.
