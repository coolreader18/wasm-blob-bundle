# wasm-blob-bundle

A crate to let you embed a blob of bytes in a wasm module without needing to
recompile anything with rustc.

## Example

```rust
// in a command line tool
use wasm_blob_bundler::BlobBundler;

let thing_to_embed: Vec<u8> = get_thing();

let module = BlobBundler::new()
    .blob("MY_THING", thing_to_embed)? // will error if thing_to_embed.len() > u32::MAX
    .build();

fs::write("thing_blob.o", module)?;
// then run wasm-ld on it but you've gotta know the exports
```

## License

This project is licensed under the MIT license. Please see the
[LICENSE](LICENSE) file for more details.
