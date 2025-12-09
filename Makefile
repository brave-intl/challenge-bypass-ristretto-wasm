all: pkg

.PHONY: clean test pack-web

pkg: cbr-wasm/src/lib.rs cbr-wasm/Cargo.toml
	cd cbr-wasm && wasm-pack build --scope brave-intl --out-dir ../pkg --target web

pack-web: pkg
	wasm-pack pack -d pkg

clean:
	rm -rf pkg

test: pkg
	node test.js
