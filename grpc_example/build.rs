extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/protos",
        includes: &[],
        input: &["protos/helloworld.proto"],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}
