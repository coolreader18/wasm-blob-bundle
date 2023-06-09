#[no_mangle]
pub fn get_foo(i: usize) -> i32 {
    FOO.get(i).map_or(-1, |&b| b as i32)
}

wasm_blob::blob!(FOO);
