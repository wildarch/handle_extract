fn main() {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(HandleExtract)]")
        .compile_protos(&["src/handle.proto", "src/test_message.proto"], &["src/"])
        .unwrap();
}
