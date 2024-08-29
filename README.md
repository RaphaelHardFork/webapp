# Frontend WASM webapp

*WASM app served by Trunk and made with Leptos*

## Requirement

Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

Install Trunk:
```bash
cargo install trunk
```

*NOTE: `nightly` features of `leptos` are not used here.*

## Usage

```bash
cd client/
trunk serve index.html
```

## Tests
*Unit test are useful for testing pure logic inside components. (see [validate_email](./client/src/utils.rs#17))*  

Unit tests:  
```bash
cargo test
```

End-to-end testing (with DOM interaction) is tested with `wasm-bindgen` & `wasm-pack`:

```bash
wasm-pack test --chrome client/
```

*[Installing wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)*
