fn main() {
    prost_build::Config::new()
        .field_type_attribute("oak.handle.Handle", "#[handle_extract]")
        .field_type_attribute("string", "#[handle_test]")
        .compile_protos(&["src/handle.proto", "src/test_message.proto"], &["src/"])
        .unwrap();
}
