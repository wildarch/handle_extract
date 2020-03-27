fn main() {
    prost_build::compile_protos(&["src/handle.proto", "src/test_message.proto"], &["src/"])
        .unwrap();
}
