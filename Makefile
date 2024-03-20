.PHONY: run

run: zig-out/lib/hello.wasm
	cargo run

hello.wasm: hello.zig
	zig build-lib -target wasm32-wasi-musl hello.zig -dynamic -rdynamic
	rm ./hello.wasm.o

zig-out/lib/hello.wasm: hello.zig build.zig build.zig.zon
	zig build
