use std::path::Path;

mod imports {
    test_helpers::codegen_spidermonkey_import!(
        // TODO: should support more of the `*.witx` test suite
        "strings.witx"
        "simple_lists.witx"
        "simple_functions.witx"
    );
}

mod exports {
    test_helpers::codegen_spidermonkey_export!(
        // TODO: should support more of the `*.witx` test suite
        "strings.witx"
        "simple_lists.witx"
        "simple_functions.witx"
    );
}

fn verify(dir: &str, _name: &str) {
    let wasm = std::fs::read(Path::new(dir).join("foo.wasm")).unwrap();
    let mut validator = wasmparser::Validator::new();
    validator.wasm_features(wasmparser::WasmFeatures {
        bulk_memory: true,
        module_linking: true,
        multi_memory: true,
        ..wasmparser::WasmFeatures::default()
    });
    validator.validate_all(&wasm).expect("wasm isn't valid");
}
