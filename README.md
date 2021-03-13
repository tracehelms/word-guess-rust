# Hangman with Rust and Yew

This implements a basic version of the game Hangman. It uses Rust and Yew to compile to Web Assembly.

## Usage

### Build

When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### Serve locally

```
yarn run dev
```

### Run tests

```
cargo test
yarn test
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
