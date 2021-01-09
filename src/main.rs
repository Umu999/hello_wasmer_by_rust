use wasmer::{imports, wat2wasm, Instance, Module, Store};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let wasm_bytes = wat2wasm(
        br#"
        (module
            (type $add_one_t (func (param i32) (result i32)))
            (func $add_one_f (type $add_one_t) (param $value i32) (result i32)
                local.get $value
                i32.const 1
                i32.add
            )
            (export "add_one" (func $add_one_f))
        )
        "#,
    )?;

    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance
        .exports
        .get_function("add_one")?
        .native::<i32, i32>()?;

    let result = add_one.call(1)?;

    println!("Results of `add_one`: {:?}", result);
    assert_eq!(result, 2);

    Ok(())
}
