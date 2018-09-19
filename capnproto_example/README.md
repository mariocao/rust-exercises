# Capn proto example

Simple example of Cap'n proto in Rust.

Example from [GitHub - capnproto/capnproto-rust: Cap'n Proto for Rust](https://github.com/capnproto/capnproto-rust).

## Install cap'n proto

Install cap'n proto, e.g.:
```bash
yay capnproto
```

## Rust example

Build project:
```bash
cargo build
```

The Rust will be generated under the `target` folder, e.g. `/target/debug/build/addressbook-<XYZ>/out/addressbook_capnp.rs`.

Then, execute example as:
```bash
cargo run write | cargo run read
```