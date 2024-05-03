.PHONY: wasm
wasm:
	cd src/core && wasm-pack build --target web

.PHONY: wasm-dev
wasm-dev:
	cd src/core && wasm-pack build --target web --dev

.PHONY: build
build:
	node esbuild.config.mjs
