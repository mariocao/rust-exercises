# Flatbuffer example

Simple guide to compile a flat buffer and execute an example in Rust.

Example from [flatbuffers/samples at master · google/flatbuffers · GitHub](https://github.com/google/flatbuffers/tree/master/samples).

## Install flat buffers compiler

Clone repository:
```bash
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make
```

## Rust example

From the current repository, compile the flat buffer example with `flatc` as:
```bash
.<PATH_TO_FLATC>/flatc -o src/ --rust ../protos/monster.fbs 
```
Execute example as:
```bash
cargo run
```