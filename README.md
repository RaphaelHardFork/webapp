# Full-stack webapp

## Requirement

`nightly` features of `leptos` are not used here.

```bash
cargo install trunk
```

## Usage

```bash
cd client/
trunk serve
```

## Tests

Components logic is tested by reproducing the logic into an unit test: 
```bash
cargo test
```

End-to-end testing (with DOM interaction) is tested with `wasm-bindgen` & `wasm-pack`:
```bash
wasm-pack test --chrome client/
```
*[Installing wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)*

