use anyhow::Result;
use wasi_common::pipe::WritePipe;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

fn main() -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();

    // Create a `Linker` which will be later used to instantiate this module.
    // Host functionality is defined by name within the `Linker`.
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .inherit_env()?
            .build();
        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using `4` for.
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_file(&engine, "zig-out/lib/hello.wasm")?;
        linker.module(&mut store, "", &module)?;

        let p = linker
            .get(&mut store, "", "test_cbor")
            .unwrap()
            .into_func()
            .unwrap()
            .typed::<(), u64>(&store)?
            .call(&mut store, ())?;

        let memory = linker
            .get(&mut store, "", "memory")
            .unwrap()
            .into_memory()
            .unwrap();

        let ptr = (p >> 32) as usize;
        let len = (p & 0xffffffff) as usize;

        let mut buf = Vec::new();
        buf.resize(len, 0);
        memory.read(&store, ptr, &mut buf).unwrap();

        let a: ciborium::Value = ciborium::from_reader(&buf[..])?;
        let bytes = a.as_map().unwrap().get(0).unwrap().1.as_bytes().unwrap();
        let name = String::from_utf8(bytes.to_vec())?;

        println!("len: {len}");
        println!("{:?}", a);
        println!("name: {name}");
    }
    // let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

    // And finally we can call the wasm!
    // hello.call(&mut store, ())?;

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Foo {
    name: String,
}
