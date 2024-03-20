.PHONY: run

run: hello.wasm
	cargo run

hello.wasm:
	zig build-lib -target wasm32-wasi-musl hello.zig -dynamic -rdynamic
	rm ./hello.wasm.o
