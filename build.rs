fn main() {
    prost_build::Config::new()
        .compile_protos(&["src/handle.proto"], &["src/"])
        .unwrap();
}
