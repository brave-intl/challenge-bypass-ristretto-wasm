all: pkg

.PHONY: clean test web pack-web

pkg: cbr-wasm/src/lib.rs cbr-wasm/Cargo.toml
	cd cbr-wasm && wasm-pack build --scope brave-intl --out-dir ../pkg --target nodejs

web: cbr-wasm/src/lib.rs cbr-wasm/Cargo.toml
	cd cbr-wasm && wasm-pack build --scope brave-intl --out-dir ../pkg-web --target web

pack-web: web
	wasm-pack pack -d pkg-web

clean:
	rm -rf pkg pkg-web

test: pkg
	node test.js
