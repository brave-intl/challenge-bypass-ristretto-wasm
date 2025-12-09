# challenge-bypass-ristretto-wasm

[challenge-bypass-ristretto](https://github.com/brave-intl/challenge-bypass-ristretto)
is a rust implemention of the privacy pass cryptographic protocol using the ristretto group.

this crate implements a webassembly wrapper to enable it to be used in a web context.

# usage warning

this crate is intended for client side use only and is not presently interoperable with
privacy pass implementations beyond challenge-bypass-ristretto and it's other bindings.

no attempts have been made made at ensuring constant time behavior when compiled to WASM and as such
at minimum it should be considered inappropriate for backend use.

# developing

install dependencies:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

rebuild nodejs package:
```
make
```

run simple test script ( requires nodejs ):
```
make test
```
