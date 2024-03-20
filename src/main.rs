use anyhow::Result;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

fn main() -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();

    // Create a `Linker` which will be later used to instantiate this module.
    // Host functionality is defined by name within the `Linker`.
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .inherit_env()?
        .build();
    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, wasi);
    let module = Module::from_file(&engine, "hello.wasm")?;
    linker.module(&mut store, "", &module)?;

    linker
        .get(&mut store, "", "hello")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    // let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

    // And finally we can call the wasm!
    // hello.call(&mut store, ())?;

    Ok(())
}
