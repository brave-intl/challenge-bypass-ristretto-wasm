all: pkg

pkg:
	cd cbr-wasm && wasm-pack build --out-dir ../pkg --target nodejs
