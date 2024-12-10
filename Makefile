.PHONY: build serve clean

build:
	cargo build --target wasm32-unknown-unknown --release

serve: build
	basic-http-server -a 127.0.0.1:8081 dist

clean:
	cargo clean
	rm -rf dist
