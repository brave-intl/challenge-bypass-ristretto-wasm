all: test

.PHONY: clean test

pkg: cbr-wasm/src/lib.rs cbr-wasm/Cargo.toml cbr-wasm/Cargo.lock
	cd cbr-wasm && wasm-pack build --scope brave-intl --out-dir ../pkg --target nodejs

clean:
	rm -rf pkg

test: pkg
	node test.js
